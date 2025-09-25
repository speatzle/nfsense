use super::ApplyError;
use crate::definitions::config::Config;
use crate::definitions::firewall::{SNATType, Verdict};
use crate::definitions::object::{Address, AddressType, PortDefinition, Service, ServiceType};
use nftables::expr;
use nftables::expr::Payload::PayloadField;
use nftables::expr::{Expression, MetaKey, NamedExpression, SetItem, CT};
use nftables::schema::NfCmd::Flush;
use nftables::schema::{FlushObject, NfListObject, NfObject, Table};
use nftables::stmt;
use nftables::stmt::{Operator, Statement};
use nftables::types::{NfChainPolicy, NfChainType, NfFamily, NfHook};
use nftables::{batch::Batch, helper, schema};
use std::collections::HashSet;
use std::io::Write;
use std::process::Command;
use tracing::{error, info};

const NFTABLES_CONFIG_PATH: &str = "/etc/nfsense/nftables.conf";
const NFTABLES_CT_DNAT_MARK_OFFSET: u32 = 1000;

fn port_definition_to_expression(port_def: &PortDefinition) -> Option<Expression<'static>> {
    match port_def {
        PortDefinition::Any => None,
        PortDefinition::Single { port } => Some(Expression::Number((*port).into())),
        PortDefinition::Range {
            start_port,
            end_port,
        } => Some(Expression::Range(Box::new(expr::Range {
            range: [
                Expression::Number((*start_port).into()),
                Expression::Number((*end_port).into()),
            ],
        }))),
    }
}

fn generate_service_matcher_statements(service: &Service) -> Vec<Statement<'static>> {
    let mut statements = Vec::new();

    match &service.service_type {
        ServiceType::TCP {
            source,
            destination,
        } => {
            // Handle TCP source port
            if let Some(right_expr) = port_definition_to_expression(source) {
                statements.push(Statement::Match(stmt::Match {
                    op: Operator::EQ,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "tcp".into(),
                            field: "sport".into(),
                        },
                    ))),
                    right: right_expr,
                }));
            }

            // Handle TCP destination port
            if let Some(right_expr) = port_definition_to_expression(destination) {
                statements.push(Statement::Match(stmt::Match {
                    op: Operator::EQ,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "tcp".into(),
                            field: "dport".into(),
                        },
                    ))),
                    right: right_expr,
                }));
            }
        }
        ServiceType::UDP {
            source,
            destination,
        } => {
            // Handle UDP source port
            if let Some(right_expr) = port_definition_to_expression(source) {
                statements.push(Statement::Match(stmt::Match {
                    op: Operator::EQ,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "udp".into(),
                            field: "sport".into(),
                        },
                    ))),
                    right: right_expr,
                }));
            }

            // Handle UDP destination port
            if let Some(right_expr) = port_definition_to_expression(destination) {
                statements.push(Statement::Match(stmt::Match {
                    op: Operator::EQ,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "udp".into(),
                            field: "dport".into(),
                        },
                    ))),
                    right: right_expr,
                }));
            }
        }
        ServiceType::ICMP { ptypes: _ } => {
            // TODO match icmp packet types
            statements.push(Statement::Match(stmt::Match {
                op: Operator::EQ,
                left: Expression::Named(NamedExpression::Payload(PayloadField(
                    expr::PayloadField {
                        protocol: "ip".into(),
                        field: "protocol".into(),
                    },
                ))),
                right: Expression::String("icmp".into()),
            }));
        }
        ServiceType::Group { members: _ } => {
            // TODO group
        }
    }

    statements
}

fn generate_address_expression(
    addresses: Vec<Address>,
    pending_config: Config,
) -> Expression<'static> {
    let mut address_set_items = Vec::new();

    for address in addresses {
        match address.clone().address_type {
            AddressType::Host { address } => {
                address_set_items.push(SetItem::Element(Expression::String(
                    address.to_string().into(),
                )));
            }
            AddressType::Network { network } => {
                address_set_items.push(SetItem::Element(Expression::Named(
                    NamedExpression::Prefix(expr::Prefix {
                        addr: Expression::String(network.network().to_string().into()).into(),
                        len: network.prefix_len().into(),
                    }),
                )));
            }
            AddressType::Range { range } => {
                _ = range;
                /* TODO Implement Range when the type is correct
                address_set_items.push(SetItem::Element(Expression::Named(
                    NamedExpression::Range(expr::Range {
                        range: Expression::String(range. .to_string().into())
                            .into(),
                        len: network.prefix_len().into(),
                    }),
                )));
                */
            }
            AddressType::Group { members } => {
                _ = pending_config;
                _ = members;
                // TODO
            }
        }
    }

    // TODO don't return set if there is only one item
    return Expression::Named(NamedExpression::Set(address_set_items));
}

pub fn apply_nftables(pending_config: Config, _current_config: Config) -> Result<(), ApplyError> {
    // TODO add a uuid to rules to be able to identify them and their counters to prevent rule reordering swapping counter values
    // Also usefull for log matching
    let mut batch = Batch::new();

    // Create table if not exists
    batch.add(NfListObject::Table(Table {
        family: NfFamily::INet,
        name: "nfsense_inet".into(),
        ..Default::default()
    }));

    // Delete all Rules
    batch.add_cmd(Flush(FlushObject::Table(Table {
        family: NfFamily::INet,
        name: "nfsense_inet".into(),
        ..Default::default()
    })));

    // Used to track all counters which should exist, after all counters are created (recreation is a noop and preserves state)
    // check if there are currently counters which should not exist and delete them
    // dont compare current and pending config since this might be a revert and the current config might not be correct or some other leftover state
    let mut counters: Vec<String> = vec![];

    // Input Chain
    batch.add(NfListObject::Chain(schema::Chain {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        name: "inbound".into(),
        _type: Some(NfChainType::Filter),
        hook: Some(NfHook::Input),
        prio: Some(0),
        policy: Some(NfChainPolicy::Drop),
        ..Default::default()
    }));

    // Inbound Connection Tracking
    batch.add(NfListObject::Rule(schema::Rule {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        chain: "inbound".into(),
        comment: Some("Allow established and related connections".into()),
        expr: vec![Statement::VerdictMap(nftables::stmt::VerdictMap {
            key: Expression::Named(NamedExpression::CT(CT {
                key: "state".into(),
                ..Default::default()
            })),
            data: Expression::Named(NamedExpression::Set(vec![
                SetItem::MappingStatement(
                    Expression::String("invalid".into()),
                    Statement::Drop(Some(stmt::Drop {})),
                ),
                SetItem::MappingStatement(
                    Expression::String("related".into()),
                    Statement::Accept(Some(stmt::Accept {})),
                ),
                SetItem::MappingStatement(
                    Expression::String("established".into()),
                    Statement::Accept(Some(stmt::Accept {})),
                ),
            ])),
        })]
        .into(),
        ..Default::default()
    }));

    // Inbound allow loopback
    batch.add(NfListObject::Rule(schema::Rule {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        chain: "inbound".into(),
        comment: Some("Allow Loopback connections".into()),
        expr: vec![
            Statement::Match(stmt::Match {
                left: Expression::Named(NamedExpression::Meta(expr::Meta {
                    key: MetaKey::Iifname,
                })),
                right: Expression::String("lo".into()),
                op: Operator::EQ,
            }),
            Statement::Accept(None),
        ]
        .into(),
        ..Default::default()
    }));

    // Inbound temp allow all, Inbound rules should go here
    batch.add(NfListObject::Rule(schema::Rule {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        chain: "inbound".into(),
        comment: Some("Temp Allow all".into()),
        expr: vec![Statement::Accept(Some(stmt::Accept {}))].into(),
        ..Default::default()
    }));

    counters.push("inbound_default_drop".to_owned());
    batch.add(NfListObject::Counter(schema::Counter {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        name: "inbound_default_drop".into(),
        ..Default::default()
    }));

    // Inbound default drop, TODO Log
    batch.add(NfListObject::Rule(schema::Rule {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        chain: "inbound".into(),
        comment: Some("Inbound default drop".into()),
        expr: vec![
            Statement::Counter(stmt::Counter::Named("inbound_default_drop".into())),
            Statement::Log(Some(stmt::Log {
                prefix: Some("inbound_default_drop".into()),
                group: None,
                snaplen: None,
                queue_threshold: None,
                level: None,
                flags: Some(HashSet::from([stmt::LogFlag::All])),
            })),
            Statement::Drop(Some(stmt::Drop {})),
        ]
        .into(),
        ..Default::default()
    }));

    // Forward Chain
    batch.add(NfListObject::Chain(schema::Chain {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        name: "forward".into(),
        _type: Some(NfChainType::Filter),
        hook: Some(NfHook::Forward),
        prio: Some(0),
        policy: Some(NfChainPolicy::Drop),
        ..Default::default()
    }));

    // Forward Connection Tracking
    batch.add(NfListObject::Rule(schema::Rule {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        chain: "forward".into(),
        comment: Some("Allow established and related connections".into()),
        expr: vec![Statement::VerdictMap(nftables::stmt::VerdictMap {
            key: Expression::Named(NamedExpression::CT(CT {
                key: "state".into(),
                ..Default::default()
            })),
            data: Expression::Named(NamedExpression::Set(vec![
                SetItem::MappingStatement(
                    Expression::String("invalid".into()),
                    Statement::Drop(Some(stmt::Drop {})),
                ),
                SetItem::MappingStatement(
                    Expression::String("related".into()),
                    Statement::Accept(Some(stmt::Accept {})),
                ),
                SetItem::MappingStatement(
                    Expression::String("established".into()),
                    Statement::Accept(Some(stmt::Accept {})),
                ),
            ])),
        })]
        .into(),
        ..Default::default()
    }));

    // Prerouting Chain
    batch.add(NfListObject::Chain(schema::Chain {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        name: "prerouting".into(),
        _type: Some(NfChainType::NAT),
        hook: Some(NfHook::Prerouting),
        prio: Some(-100),
        policy: Some(NfChainPolicy::Accept),
        ..Default::default()
    }));

    // TODO create dnat rules and their automatic forward rules
    for (index, res) in (0u32..).zip(
        pending_config
            .firewall
            .destination_nat_rules
            .iter()
            .enumerate(),
    ) {
        let (_, rule) = res;

        if rule.counter {
            counters.push(format!("dnat_{}", index));
            batch.add(NfListObject::Counter(schema::Counter {
                family: NfFamily::INet,
                table: "nfsense_inet".into(),
                name: format!("dnat_{}", index).into(),
                ..Default::default()
            }));
        }

        let source_addresses = rule.source_addresses(pending_config.clone());
        let destination_addresses = rule.destination_addresses(pending_config.clone());

        // TODO find a way to have only one rule? icmp clashes with tcp/udp
        // also needs to run when there are no services
        for service in rule.services(pending_config.clone()) {
            let mut expression: Vec<Statement> = vec![];

            // Source Address matchers
            if source_addresses.len() > 0 {
                let mut source_op = Operator::EQ;
                if rule.negate_source {
                    source_op = Operator::NEQ;
                }

                expression.push(Statement::Match(stmt::Match {
                    op: source_op,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "ip".into(),
                            field: "saddr".into(),
                        },
                    ))),
                    right: generate_address_expression(
                        source_addresses.clone(),
                        pending_config.clone(),
                    ),
                }));
            }

            // Destination Address matchers
            if destination_addresses.len() > 0 {
                let mut source_op = Operator::EQ;
                if rule.negate_destination {
                    source_op = Operator::NEQ;
                }

                expression.push(Statement::Match(stmt::Match {
                    op: source_op,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "ip".into(),
                            field: "daddr".into(),
                        },
                    ))),
                    right: generate_address_expression(
                        destination_addresses.clone(),
                        pending_config.clone(),
                    ),
                }));
            }

            // Generate service matcher statements
            expression.extend(generate_service_matcher_statements(&service));

            if rule.counter {
                expression.push(Statement::Counter(stmt::Counter::Named(
                    format!("dnat_{}", index).into(),
                )));
            }

            if rule.log {
                expression.push(Statement::Log(Some(stmt::Log {
                    prefix: Some(format!("dnat_{}", index).into()),
                    group: None,
                    snaplen: None,
                    queue_threshold: None,
                    level: None,
                    flags: Some(HashSet::from([stmt::LogFlag::All])),
                })))
            }

            if rule.automatic_forward_rule {
                expression.push(Statement::Mangle(stmt::Mangle {
                    key: Expression::Named(NamedExpression::CT(CT {
                        key: "mark".into(),
                        ..Default::default()
                    })),
                    // TODO set a max rule count?
                    value: Expression::Number(index + NFTABLES_CT_DNAT_MARK_OFFSET),
                }));
            }

            let mut addr = None;
            let mut destination_port = None;

            if let Some(dnataddr) = rule.dnat_address(pending_config.clone()) {
                match dnataddr.address_type {
                    AddressType::Host { address } => {
                        addr = Some(Expression::String(address.to_string().into()));
                    }
                    _ => todo!("Unsupported address type"),
                }
            }

            if let Some(dnatservice) = rule.dnat_service(pending_config.clone()) {
                match dnatservice.service_type {
                    ServiceType::TCP { destination, .. } => match destination {
                        PortDefinition::Single { port } => {
                            destination_port = Some(Expression::Number(port))
                        }
                        _ => todo!("Unsupported port definition"),
                    },
                    ServiceType::UDP { destination, .. } => match destination {
                        PortDefinition::Single { port } => {
                            destination_port = Some(Expression::Number(port))
                        }
                        _ => todo!("Unsupported port definition"),
                    },
                    _ => todo!("Unsupported service type"),
                }
            }

            expression.push(Statement::DNAT(Some(stmt::NAT {
                addr: addr,
                port: destination_port,
                // TODO ipv6
                family: Some(stmt::NATFamily::IP),
                flags: None,
            })));

            batch.add(NfListObject::Rule(schema::Rule {
                family: NfFamily::INet,
                table: "nfsense_inet".into(),
                chain: "prerouting".into(),
                comment: Some(rule.name.clone().into()),
                expr: expression.into(),
                ..Default::default()
            }));
        }

        // Automatic forward rule
        if rule.automatic_forward_rule {
            // in prerouting rule: ip saddr 192.168.1.100 ct mark set 0x1
            // in forward rule: ct mark 0x1 accept
            let mut forward_expression: Vec<Statement> = vec![];
            forward_expression.push(Statement::Match(stmt::Match {
                op: Operator::EQ,
                left: Expression::Named(NamedExpression::CT(CT {
                    key: "mark".into(),
                    ..Default::default()
                })),
                // TODO set a max rule count?
                right: Expression::Number(index + NFTABLES_CT_DNAT_MARK_OFFSET),
            }));
            if rule.log {
                forward_expression.push(Statement::Log(Some(stmt::Log {
                    prefix: Some(format!("fw_dnat_{}", index).into()),
                    group: None,
                    snaplen: None,
                    queue_threshold: None,
                    level: None,
                    flags: Some(HashSet::from([stmt::LogFlag::All])),
                })))
            }
            forward_expression.push(Statement::Accept(None));

            batch.add(NfListObject::Rule(schema::Rule {
                family: NfFamily::INet,
                table: "nfsense_inet".into(),
                chain: "forward".into(),
                comment: Some(format!("Automatic Forward Rule: {} ", rule.name.clone()).into()),
                expr: forward_expression.into(),
                ..Default::default()
            }));
        }
    }

    // Postrouting Chain
    batch.add(NfListObject::Chain(schema::Chain {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        name: "postrouting".into(),
        _type: Some(NfChainType::NAT),
        hook: Some(NfHook::Postrouting),
        prio: Some(100),
        policy: Some(NfChainPolicy::Accept),
        ..Default::default()
    }));

    // Create snat rules and their automatic forward rules
    for (index, res) in (0u32..).zip(pending_config.firewall.source_nat_rules.iter().enumerate()) {
        let (_, rule) = res;

        if rule.counter {
            counters.push(format!("snat_{}", index));
            batch.add(NfListObject::Counter(schema::Counter {
                family: NfFamily::INet,
                table: "nfsense_inet".into(),
                name: format!("snat_{}", index).into(),
                ..Default::default()
            }));
        }

        let source_addresses = rule.source_addresses(pending_config.clone());
        let destination_addresses = rule.destination_addresses(pending_config.clone());

        // TODO find a way to have only one rule? icmp clashes with tcp/udp
        // also needs to run when there are no services
        for service in rule.services(pending_config.clone()) {
            // Gets reused in the automatic forward rule
            let mut match_expression: Vec<Statement> = vec![];

            // Source Address matchers
            if source_addresses.len() > 0 {
                let mut source_op = Operator::EQ;
                if rule.negate_source {
                    source_op = Operator::NEQ;
                }

                match_expression.push(Statement::Match(stmt::Match {
                    op: source_op,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "ip".into(),
                            field: "saddr".into(),
                        },
                    ))),
                    right: generate_address_expression(
                        source_addresses.clone(),
                        pending_config.clone(),
                    ),
                }));
            }

            // Destination Address matchers
            if destination_addresses.len() > 0 {
                let mut source_op = Operator::EQ;
                if rule.negate_destination {
                    source_op = Operator::NEQ;
                }

                match_expression.push(Statement::Match(stmt::Match {
                    op: source_op,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "ip".into(),
                            field: "daddr".into(),
                        },
                    ))),
                    right: generate_address_expression(
                        destination_addresses.clone(),
                        pending_config.clone(),
                    ),
                }));
            }

            // Generate service matcher statements
            match_expression.extend(generate_service_matcher_statements(&service));

            let mut expression: Vec<Statement> = vec![];
            expression.extend(match_expression.clone());

            if rule.counter {
                expression.push(Statement::Counter(stmt::Counter::Named(
                    format!("snat_{}", index).into(),
                )));
            }

            if rule.log {
                expression.push(Statement::Log(Some(stmt::Log {
                    prefix: Some(format!("snat_{}", index).into()),
                    group: None,
                    snaplen: None,
                    queue_threshold: None,
                    level: None,
                    flags: Some(HashSet::from([stmt::LogFlag::All])),
                })))
            }

            match rule.snat_type {
                SNATType::Masquerade => {
                    expression.push(Statement::Masquerade(None));
                }
                SNATType::SNAT { .. } => {
                    let mut source_addr = None;
                    let mut source_port = None;

                    if let Some(address) = rule.snat_type.address(pending_config.clone()) {
                        match address.address_type {
                            AddressType::Host { address } => {
                                source_addr = Some(Expression::String(address.to_string().into()));
                            }
                            _ => todo!("Unsupported address type"),
                        }
                    }

                    if let Some(service) = rule.snat_type.service(pending_config.clone()) {
                        match service.service_type {
                            ServiceType::TCP { destination, .. } => match destination {
                                PortDefinition::Single { port } => {
                                    source_port = Some(Expression::Number(port))
                                }
                                _ => todo!("Unsupported port definition: {}", service.name),
                            },
                            ServiceType::UDP { destination, .. } => match destination {
                                PortDefinition::Single { port } => {
                                    source_port = Some(Expression::Number(port))
                                }
                                _ => todo!("Unsupported port definition: {}", service.name),
                            },
                            _ => todo!("Unsupported service type: {}", service.name),
                        }
                    }

                    expression.push(Statement::SNAT(Some(stmt::NAT {
                        addr: source_addr,
                        port: source_port,
                        // TODO ipv6
                        family: Some(stmt::NATFamily::IP),
                        flags: None,
                    })));
                }
            }

            batch.add(NfListObject::Rule(schema::Rule {
                family: NfFamily::INet,
                table: "nfsense_inet".into(),
                chain: "postrouting".into(),
                comment: Some(rule.name.clone().into()),
                expr: expression.into(),
                ..Default::default()
            }));

            // Automatic forward rule
            if rule.automatic_forward_rule {
                let mut forward_expression: Vec<Statement> = vec![];
                forward_expression.extend(match_expression);

                if rule.log {
                    forward_expression.push(Statement::Log(Some(stmt::Log {
                        prefix: Some(format!("fw_snat_{}", index).into()),
                        group: None,
                        snaplen: None,
                        queue_threshold: None,
                        level: None,
                        flags: Some(HashSet::from([stmt::LogFlag::All])),
                    })))
                }
                forward_expression.push(Statement::Accept(None));

                batch.add(NfListObject::Rule(schema::Rule {
                    family: NfFamily::INet,
                    table: "nfsense_inet".into(),
                    chain: "forward".into(),
                    comment: Some(format!("Automatic Forward Rule: {} ", rule.name.clone()).into()),
                    expr: forward_expression.into(),
                    ..Default::default()
                }));
            }
        }
    }

    // TODO create normal forward rules
    for (index, res) in (0u32..).zip(pending_config.firewall.forward_rules.iter().enumerate()) {
        let (_, rule) = res;

        if rule.counter {
            counters.push(format!("fw_{}", index));
            batch.add(NfListObject::Counter(schema::Counter {
                family: NfFamily::INet,
                table: "nfsense_inet".into(),
                name: format!("fw_{}", index).into(),
                ..Default::default()
            }));
        }

        let source_addresses = rule.source_addresses(pending_config.clone());
        let destination_addresses = rule.destination_addresses(pending_config.clone());

        // TODO find a way to have only one rule? icmp clashes with tcp/udp
        // also needs to run when there are no services
        for service in rule.services(pending_config.clone()) {
            // Gets reused in the automatic forward rule
            let mut match_expression: Vec<Statement> = vec![];

            // Source Address matchers
            if source_addresses.len() > 0 {
                let mut source_op = Operator::EQ;
                if rule.negate_source {
                    source_op = Operator::NEQ;
                }

                match_expression.push(Statement::Match(stmt::Match {
                    op: source_op,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "ip".into(),
                            field: "saddr".into(),
                        },
                    ))),
                    right: generate_address_expression(
                        source_addresses.clone(),
                        pending_config.clone(),
                    ),
                }));
            }

            // Destination Address matchers
            if destination_addresses.len() > 0 {
                let mut source_op = Operator::EQ;
                if rule.negate_destination {
                    source_op = Operator::NEQ;
                }

                match_expression.push(Statement::Match(stmt::Match {
                    op: source_op,
                    left: Expression::Named(NamedExpression::Payload(PayloadField(
                        expr::PayloadField {
                            protocol: "ip".into(),
                            field: "daddr".into(),
                        },
                    ))),
                    right: generate_address_expression(
                        destination_addresses.clone(),
                        pending_config.clone(),
                    ),
                }));
            }

            // Generate service matcher statements
            match_expression.extend(generate_service_matcher_statements(&service));

            let mut expression: Vec<Statement> = vec![];
            expression.extend(match_expression.clone());

            if rule.counter {
                expression.push(Statement::Counter(stmt::Counter::Named(
                    format!("fw_{}", index).into(),
                )));
            }

            if rule.log {
                expression.push(Statement::Log(Some(stmt::Log {
                    prefix: Some(format!("fw_{}", index).into()),
                    group: None,
                    snaplen: None,
                    queue_threshold: None,
                    level: None,
                    flags: Some(HashSet::from([stmt::LogFlag::All])),
                })))
            }

            expression.push(match rule.verdict {
                Verdict::Accept => Statement::Accept(None),
                Verdict::Drop => Statement::Drop(None),
                Verdict::Continue => Statement::Continue(None),
            });

            batch.add(NfListObject::Rule(schema::Rule {
                family: NfFamily::INet,
                table: "nfsense_inet".into(),
                chain: "forward".into(),
                comment: Some(rule.name.clone().into()),
                expr: expression.into(),
                ..Default::default()
            }));
        }
    }

    counters.push("fw_default_drop".to_owned());
    batch.add(NfListObject::Counter(schema::Counter {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        name: "fw_default_drop".into(),
        ..Default::default()
    }));

    // Forward default drop counter, TODO Log
    batch.add(NfListObject::Rule(schema::Rule {
        family: NfFamily::INet,
        table: "nfsense_inet".into(),
        chain: "forward".into(),
        comment: Some("Forward default drop".into()),
        expr: vec![
            Statement::Counter(stmt::Counter::Named("fw_default_drop".into())),
            Statement::Log(Some(stmt::Log {
                prefix: Some("fw_default_drop".into()),
                group: None,
                snaplen: None,
                queue_threshold: None,
                level: None,
                flags: Some(HashSet::from([stmt::LogFlag::All])),
            })),
            Statement::Drop(Some(stmt::Drop {})),
        ]
        .into(),
        ..Default::default()
    }));

    // TODO save current counter state in case of disaster (not when reverting!)

    // Get existing counters
    let current_counters = helper::get_current_ruleset_with_args(
        helper::DEFAULT_NFT,
        vec!["list", "counters", "table", "inet", "nfsense_inet"],
    )
    .unwrap();

    // Check each current counter and delete those that shouldn't exist
    for cc in current_counters.objects.iter() {
        match cc {
            NfObject::ListObject(NfListObject::Counter(ref counter)) => {
                let mut should_exist = false;

                // Check if this counter should exist in our expected counters list
                for expected_counter in &counters {
                    if counter.name == *expected_counter {
                        should_exist = true;
                        break;
                    }
                }

                // Delete counter if it shouldn't exist
                if !should_exist {
                    info!("Deleting counter {}", counter.name);
                    batch.delete(NfListObject::Counter(schema::Counter {
                        family: NfFamily::INet,
                        table: "nfsense_inet".into(),
                        name: counter.name.clone(),
                        ..Default::default()
                    }));
                }
            }
            _ => {}
        }
    }

    let ruleset = batch.to_nftables();

    // Check if ruleset is valid, TODO fix unwarp
    info!("Checking Ruleset");
    match helper::apply_ruleset_with_args(&ruleset, helper::DEFAULT_NFT, vec!["-c"]) {
        Ok(_) => info!("Ruleset Ok"),
        Err(e) => {
            error!("Config check failed: {}", e);
            return Err(ApplyError::ConfigCheckFailed);
        }
    }

    // Apply ruleset,TODO fix unwarp
    info!("Applying Ruleset");
    match helper::apply_ruleset(&ruleset) {
        Ok(_) => info!("Ruleset Applied"),
        Err(e) => {
            error!("Config Apply failed: {}", e);
            return Err(ApplyError::ConfigApplyFailed);
        }
    }

    info!("Save new Ruleset to disk");
    std::fs::remove_file(NFTABLES_CONFIG_PATH)?;
    let mut f = std::fs::File::create(NFTABLES_CONFIG_PATH)?;
    f.write_all(
        "#!/usr/sbin/nft -f\n\n\n# Nftables config for restore on reboot\nflush ruleset\n"
            .as_bytes(),
    )?;

    let mut cmd = Command::new("nft")
        .arg("list")
        .arg("ruleset")
        .stdout(f)
        .spawn()?;

    match cmd.wait() {
        Ok(out) => {
            if out.success() {
                info!("New Ruleset Saved");
                Ok(())
            } else {
                Err(ApplyError::ConfigApplyFailed)
            }
        }
        Err(err) => Err(ApplyError::IOError(err)),
    }
}
