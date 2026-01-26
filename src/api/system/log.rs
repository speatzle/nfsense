use crate::state::RpcState;
use jsonrpsee::core::SubscriptionError;
use jsonrpsee::types::Params;
use jsonrpsee::{
    Extensions, PendingSubscriptionSink, RpcModule, SubscriptionMessage, SubscriptionSink,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use tracing::{error, trace};

const ULOG_SOCKET_PATH: &str = "/run/ulog/ulogd.socket";

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_subscription(
            "system.logs.fw.live.subscribe",
            "system.logs.fw.live.event",
            "system.logs.fw.live.unsubscribe",
            subscribe_fw_live_log,
        )
        .unwrap();
}

#[derive(Serialize, Clone)]
#[serde(tag = "ip.protocol")]
enum LogEntry {
    ICMP(LogICMP),
    TCP(LogTCP),
    UDP(LogUDP),
    Unknown(LogUnknown),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LogICMP {
    timestamp: String,
    #[serde(rename = "oob.prefix")]
    prefix: String,
    #[serde(rename = "oob.family")]
    family: u32,
    #[serde(rename = "oob.in")]
    input_interface: String,
    #[serde(rename = "oob.out")]
    output_interface: String,
    #[serde(rename = "ip.ttl")]
    ttl: u32,
    #[serde(rename = "src_ip")]
    source_ip: String,
    #[serde(rename = "dest_ip")]
    destination_ip: String,
    #[serde(rename = "mac.saddr.str")]
    source_mac: String,
    #[serde(rename = "mac.daddr.str")]
    destination_mac: String,
    #[serde(rename = "icmp.type")]
    _type: u32,
    #[serde(rename = "icmp.code")]
    code: u32,
    #[serde(rename = "icmp.echoid")]
    echo_id: u32,
    #[serde(rename = "icmp.echoseq")]
    echo_sequence: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LogTCP {
    timestamp: String,
    #[serde(rename = "oob.prefix")]
    prefix: String,
    #[serde(rename = "oob.family")]
    family: u32,
    #[serde(rename = "oob.in")]
    input_interface: String,
    #[serde(rename = "oob.out")]
    output_interface: String,
    #[serde(rename = "ip.ttl")]
    ttl: u32,
    #[serde(rename = "src_ip")]
    source_ip: String,
    #[serde(rename = "dest_ip")]
    destination_ip: String,
    #[serde(rename = "mac.saddr.str")]
    source_mac: String,
    #[serde(rename = "mac.daddr.str")]
    destination_mac: String,
    #[serde(rename = "src_port")]
    source_port: u32,
    #[serde(rename = "dest_port")]
    destination_port: u32,
    #[serde(rename = "tcp.window")]
    tcp_window: u32,
    #[serde(rename = "tcp.offset")]
    tcp_offset: u32,
    #[serde(rename = "tcp.reserved")]
    tcp_reserved: u32,
    #[serde(rename = "tcp.urg")]
    tcp_urg: u32,
    #[serde(rename = "tcp.ack")]
    tcp_ack: u32,
    #[serde(rename = "tcp.psh")]
    tcp_psh: u32,
    #[serde(rename = "tcp.rst")]
    tcp_rst: u32,
    #[serde(rename = "tcp.syn")]
    tcp_syn: u32,
    #[serde(rename = "tcp.fin")]
    typ_fin: u32,
    #[serde(rename = "tcp.csum")]
    typ_checksum: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LogUDP {
    timestamp: String,
    #[serde(rename = "oob.prefix")]
    prefix: String,
    #[serde(rename = "oob.family")]
    family: u32,
    #[serde(rename = "oob.in")]
    input_interface: String,
    #[serde(rename = "oob.out")]
    output_interface: String,
    #[serde(rename = "ip.ttl")]
    ttl: u32,
    #[serde(rename = "src_ip")]
    source_ip: String,
    #[serde(rename = "dest_ip")]
    destination_ip: String,
    #[serde(rename = "mac.saddr.str")]
    source_mac: String,
    #[serde(rename = "mac.daddr.str")]
    destination_mac: String,
    #[serde(rename = "src_port")]
    source_port: u32,
    #[serde(rename = "dest_port")]
    destination_port: u32,
    #[serde(rename = "udp.len")]
    udp_length: u32,
    #[serde(rename = "udp.csum")]
    udp_checksum: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LogUnknown {
    #[serde(rename = "ip.protocol")]
    protocol: u32,
    timestamp: String,
    #[serde(rename = "oob.prefix")]
    prefix: String,
    #[serde(rename = "oob.family")]
    family: u32,
    #[serde(rename = "oob.in")]
    input_interface: String,
    #[serde(rename = "oob.out")]
    output_interface: String,
    #[serde(rename = "ip.ttl")]
    ttl: u32,
    #[serde(rename = "src_ip")]
    source_ip: String,
    #[serde(rename = "dest_ip")]
    destination_ip: String,
    #[serde(rename = "mac.saddr.str")]
    source_mac: String,
    #[serde(rename = "mac.daddr.str")]
    destination_mac: String,
}

impl<'de> serde::Deserialize<'de> for LogEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;

        Ok(
            match value.get("ip.protocol").and_then(Value::as_u64).unwrap() {
                1 => LogEntry::ICMP(LogICMP::deserialize(value).unwrap()),
                6 => LogEntry::TCP(LogTCP::deserialize(value).unwrap()),
                17 => LogEntry::UDP(LogUDP::deserialize(value).unwrap()),
                _ => LogEntry::Unknown(LogUnknown::deserialize(value).unwrap()),
            },
        )
    }
}

pub async fn subscribe_fw_live_log<'a>(
    _: Params<'a>,
    pending_sink: PendingSubscriptionSink,
    _state: Arc<RpcState>,
    _: Extensions,
) -> Result<(), SubscriptionError> {
    let sink = match pending_sink.accept().await {
        Ok(sink) => sink,
        Err(err) => {
            error!("Failed to accept subscription: {}", err);
            return Err(SubscriptionError::from(format!(
                "Failed to accept subscription: {}",
                err
            )));
        }
    };

    trace!("Spawning Thread for live log");
    tokio::spawn(live_log_sink(sink));

    Ok(())
}

async fn live_log_sink(sink: SubscriptionSink) {
    trace!(
        "Accepted subscription with ID: {:?}",
        sink.subscription_id().to_owned()
    );

    let file = File::open(ULOG_SOCKET_PATH).expect("couldn't open file");

    for line in BufReader::new(file).lines() {
        let line = line.expect("couldn't get line");
        trace!("Received log line");
        if !line.starts_with("{") {
            trace!("Log line does not start with {{ skipping...");
            continue;
        }
        let log_entry: LogEntry = serde_json::from_str(&line).expect("couldn't deserialize");

        let message = match SubscriptionMessage::new(
            sink.method_name(),
            sink.subscription_id(),
            &log_entry,
        ) {
            Ok(msg) => msg,
            Err(err) => {
                error!("Failed to create subscription message: {}", err);
                return;
            }
        };
        trace!("Sending log entry to sink");
        match sink.send(message).await {
            Ok(_) => {}
            Err(err) => {
                error!("Subscription/ Connection was closed: {}", err);
                return;
            }
        }
    }
}
