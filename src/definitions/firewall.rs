use super::config::Config;
use super::object::{Address, Service};
use garde::Validate;
use serde::{Deserialize, Serialize};
use structdb_macros::StructDb;
use uuid::Uuid;

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Default, Debug)]
#[garde(context(Config))]
pub struct Firewall {
    #[collection]
    #[garde(dive)]
    pub forward_rules: Vec<ForwardRule>,
    #[collection]
    #[garde(dive)]
    pub destination_nat_rules: Vec<DestinationNATRule>,
    #[collection]
    #[garde(dive)]
    pub source_nat_rules: Vec<SourceNATRule>,
    #[collection]
    #[garde(dive)]
    pub inbound_rules: Vec<InboundRule>,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct ForwardRule {
    pub name: String,
    #[garde(skip)]
    #[serde(default)]
    pub uuid: Uuid,
    #[requires(Service)]
    pub services: Vec<String>,
    #[requires(Address)]
    pub source_addresses: Vec<String>,
    pub negate_source: bool,
    #[requires(Address)]
    pub destination_addresses: Vec<String>,
    pub negate_destination: bool,
    pub comment: String,
    pub counter: bool,
    pub log: bool,
    pub verdict: Verdict,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct DestinationNATRule {
    pub name: String,
    #[garde(skip)]
    #[serde(default)]
    pub uuid: Uuid,
    #[requires(Service)]
    pub services: Vec<String>,
    #[requires(Address)]
    pub source_addresses: Vec<String>,
    pub negate_source: bool,
    #[requires(Address)]
    pub destination_addresses: Vec<String>,
    pub negate_destination: bool,
    pub comment: String,
    pub automatic_forward_rule: bool,
    pub counter: bool,
    pub log: bool,
    #[requires(Address)]
    pub dnat_address: Option<String>,
    #[requires(Service)]
    pub dnat_service: Option<String>,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct SourceNATRule {
    pub name: String,
    #[garde(skip)]
    #[serde(default)]
    pub uuid: Uuid,
    #[requires(Service)]
    pub services: Vec<String>,
    #[requires(Address)]
    pub source_addresses: Vec<String>,
    pub negate_source: bool,
    #[requires(Address)]
    pub destination_addresses: Vec<String>,
    pub negate_destination: bool,
    pub comment: String,
    pub automatic_forward_rule: bool,
    pub counter: bool,
    pub log: bool,
    #[delegate]
    pub snat_type: SNATType,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
#[structdb(key = "name")]
pub struct InboundRule {
    pub name: String,
    #[garde(skip)]
    #[serde(default)]
    pub uuid: Uuid,
    #[requires(Service)]
    pub services: Vec<String>,
    #[requires(Address)]
    pub source_addresses: Vec<String>,
    pub negate_source: bool,
    #[requires(Address)]
    pub destination_addresses: Vec<String>,
    pub negate_destination: bool,
    pub comment: String,
    pub counter: bool,
    pub log: bool,
    pub verdict: Verdict,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Accept,
    Drop,
    Continue,
}

#[derive(StructDb, Serialize, Deserialize, Clone, Validate, Debug, Default, PartialEq)]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub struct SNAT {
    #[requires(Address)]
    pub address: Option<String>,
    #[requires(Service)]
    pub service: Option<String>,
}

#[derive(StructDb, Validate, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[garde(context(Config))]
#[garde(allow_unvalidated)]
pub enum SNATType {
    SNAT(SNAT),
    Masquerade,
}

#[derive(Serialize, Clone, Debug)]
pub struct RuleWithCounts<T: Serialize + Clone> {
    #[serde(flatten)]
    pub rule: T,
    pub counter_packets: i64,
    pub counter_bytes: i64,
}
