use crate::state::RpcState;
use jsonrpsee::core::SubscriptionError;
use jsonrpsee::types::Params;
use jsonrpsee::{Extensions, PendingSubscriptionSink, RpcModule, SubscriptionMessage};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use tracing::{error, info};

const ULOG_SOCKET_PATH: &str = "/run/ulog/ulogd.socket";

pub fn register_methods(module: &mut RpcModule<RpcState>) {
    module
        .register_subscription(
            "system.log.fw.live.subscribe",
            "system.log.fw.live.event",
            "system.log.fw.live.unsubscribe",
            read_live_log,
        )
        .unwrap();
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "ip.protocol")]
enum LogEntry {
    LogICMP {
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
    },
    LogTCP {
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
    },
    LogUDP {
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
    },
}

pub async fn read_live_log<'a>(
    _: Params<'a>,
    pending_sink: PendingSubscriptionSink,
    _state: Arc<RpcState>,
    _: Extensions,
) -> Result<(), SubscriptionError> {
    let file = File::open(ULOG_SOCKET_PATH).expect("couldn't open file");

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

    for line in BufReader::new(file).lines() {
        let line = line.expect("couldn't get line");
        let log_entry: LogEntry = serde_json::from_str(&line).expect("couldn't deserialize");

        match log_entry.clone() {
            LogEntry::LogICMP { prefix, .. } => {
                info!("ICMP Log: {:?}", prefix);
            }
            LogEntry::LogTCP { prefix, .. } => {
                info!("TCP Log: {:?}", prefix);
            }
            LogEntry::LogUDP { prefix, .. } => {
                info!("UDP Log: {:?}", prefix);
            }
        }

        let message = match SubscriptionMessage::new(
            sink.method_name(),
            sink.subscription_id(),
            &log_entry,
        ) {
            Ok(msg) => msg,
            Err(err) => {
                error!("Failed to create subscription message: {}", err);
                return Err(SubscriptionError::from(format!(
                    "Failed to create subscription message: {}",
                    err
                )));
            }
        };

        match sink.send(message).await {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to send log entry: {}", err);
                return Err(SubscriptionError::from(format!(
                    "Failed to send log entry: {}",
                    err
                )));
            }
        }
    }
    Ok(())
}
