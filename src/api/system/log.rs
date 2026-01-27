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
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::broadcast;
use tracing::{error, info, trace, warn};

const ULOG_SOCKET_PATH: &str = "/run/ulog/ulogd.socket";
const CHANNEL_CAPACITY: usize = 2000;

static LOG_BROADCASTER: Mutex<Option<broadcast::Sender<LogEntry>>> = Mutex::new(None);

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
#[serde(tag = "protocol")]
enum LogEntry {
    ICMP(LogICMP),
    TCP(LogTCP),
    UDP(LogUDP),
    Unknown(LogUnknown),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LogICMP {
    timestamp: String,
    #[serde(alias = "oob.prefix")]
    prefix: String,
    #[serde(alias = "oob.family")]
    family: u32,
    #[serde(alias = "oob.in")]
    input_interface: String,
    #[serde(alias = "oob.out")]
    output_interface: String,
    #[serde(alias = "ip.ttl")]
    ttl: u32,
    #[serde(alias = "src_ip")]
    source_ip: String,
    #[serde(alias = "dest_ip")]
    destination_ip: String,
    #[serde(alias = "mac.saddr.str")]
    source_mac: String,
    #[serde(alias = "mac.daddr.str")]
    destination_mac: String,
    #[serde(alias = "icmp.type")]
    _type: u32,
    #[serde(alias = "icmp.code")]
    code: u32,
    #[serde(alias = "icmp.echoid")]
    echo_id: u32,
    #[serde(alias = "icmp.echoseq")]
    echo_sequence: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LogTCP {
    timestamp: String,
    #[serde(alias = "oob.prefix")]
    prefix: String,
    #[serde(alias = "oob.family")]
    family: u32,
    #[serde(alias = "oob.in")]
    input_interface: String,
    #[serde(alias = "oob.out")]
    output_interface: String,
    #[serde(alias = "ip.ttl")]
    ttl: u32,
    #[serde(alias = "src_ip")]
    source_ip: String,
    #[serde(alias = "dest_ip")]
    destination_ip: String,
    #[serde(alias = "mac.saddr.str")]
    source_mac: String,
    #[serde(alias = "mac.daddr.str")]
    destination_mac: String,
    #[serde(alias = "src_port")]
    source_port: u32,
    #[serde(alias = "dest_port")]
    destination_port: u32,
    #[serde(alias = "tcp.window")]
    tcp_window: u32,
    #[serde(alias = "tcp.offset")]
    tcp_offset: u32,
    #[serde(alias = "tcp.reserved")]
    tcp_reserved: u32,
    #[serde(alias = "tcp.urg")]
    tcp_urg: u32,
    #[serde(alias = "tcp.ack")]
    tcp_ack: u32,
    #[serde(alias = "tcp.psh")]
    tcp_psh: u32,
    #[serde(alias = "tcp.rst")]
    tcp_rst: u32,
    #[serde(alias = "tcp.syn")]
    tcp_syn: u32,
    #[serde(alias = "tcp.fin")]
    typ_fin: u32,
    #[serde(alias = "tcp.csum")]
    typ_checksum: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LogUDP {
    timestamp: String,
    #[serde(alias = "oob.prefix")]
    prefix: String,
    #[serde(alias = "oob.family")]
    family: u32,
    #[serde(alias = "oob.in")]
    input_interface: String,
    #[serde(alias = "oob.out")]
    output_interface: String,
    #[serde(alias = "ip.ttl")]
    ttl: u32,
    #[serde(alias = "src_ip")]
    source_ip: String,
    #[serde(alias = "dest_ip")]
    destination_ip: String,
    #[serde(alias = "mac.saddr.str")]
    source_mac: String,
    #[serde(alias = "mac.daddr.str")]
    destination_mac: String,
    #[serde(alias = "src_port")]
    source_port: u32,
    #[serde(alias = "dest_port")]
    destination_port: u32,
    #[serde(alias = "udp.len")]
    udp_length: u32,
    #[serde(alias = "udp.csum")]
    udp_checksum: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LogUnknown {
    #[serde(alias = "ip.protocol")]
    protocol: u32,
    timestamp: String,
    #[serde(alias = "oob.prefix")]
    prefix: String,
    #[serde(alias = "oob.family")]
    family: u32,
    #[serde(alias = "oob.in")]
    input_interface: String,
    #[serde(alias = "oob.out")]
    output_interface: String,
    #[serde(alias = "ip.ttl")]
    ttl: u32,
    #[serde(alias = "src_ip")]
    source_ip: String,
    #[serde(alias = "dest_ip")]
    destination_ip: String,
    #[serde(alias = "mac.saddr.str")]
    source_mac: String,
    #[serde(alias = "mac.daddr.str")]
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

fn get_or_spawn_broadcaster() -> broadcast::Sender<LogEntry> {
    let mut guard = LOG_BROADCASTER.lock().unwrap();

    if let Some(tx) = &*guard {
        return tx.clone();
    }

    let (tx, _) = broadcast::channel(CHANNEL_CAPACITY);
    let tx_for_thread = tx.clone();

    trace!("Spawning dedicated OS thread for log reading");

    // OS thread as to not block tokio
    thread::spawn(move || {
        log_reader_thread(tx_for_thread);
    });

    *guard = Some(tx.clone());
    tx
}

fn log_reader_thread(tx: broadcast::Sender<LogEntry>) {
    let file = match File::open(ULOG_SOCKET_PATH) {
        Ok(f) => f,
        Err(e) => {
            error!("Could not open log socket {}: {}", ULOG_SOCKET_PATH, e);
            // Reset Mutex so we can start again
            let mut guard = LOG_BROADCASTER.lock().unwrap();
            *guard = None;
            return;
        }
    };

    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                warn!("Error reading log line: {}", e);
                break;
            }
        };

        // If we don't have a complete line, skip it
        if !line.starts_with("{") {
            continue;
        }

        if let Ok(log_entry) = serde_json::from_str::<LogEntry>(&line) {
            if tx.send(log_entry).is_err() {
                let mut guard = LOG_BROADCASTER.lock().unwrap();
                if tx.receiver_count() == 0 {
                    info!("No log subscribers left. Stopping dedicated reader thread.");
                    *guard = None;
                    return;
                }
            }
        }
    }

    let mut guard = LOG_BROADCASTER.lock().unwrap();
    *guard = None;
    warn!("Log reader thread exited (EOF/Pipe Closed)");
}

pub async fn subscribe_fw_live_log<'a>(
    _: Params<'a>,
    pending_sink: PendingSubscriptionSink,
    _state: Arc<RpcState>,
    _: Extensions,
) -> Result<(), SubscriptionError> {
    let tx = get_or_spawn_broadcaster();
    let rx = tx.subscribe();
    info!("Log Receiver count: {}", tx.receiver_count());

    let sink = match pending_sink.accept().await {
        Ok(sink) => sink,
        Err(err) => return Err(SubscriptionError::from(err.to_string())),
    };

    tokio::spawn(forward_logs_to_sink(sink, rx));
    Ok(())
}

async fn forward_logs_to_sink(sink: SubscriptionSink, mut rx: broadcast::Receiver<LogEntry>) {
    loop {
        tokio::select! {
            val = rx.recv() => {
                match val {
                    Ok(log_entry) => {
                        let msg = match SubscriptionMessage::new(
                            sink.method_name(),
                            sink.subscription_id(),
                            &log_entry,
                        ) {
                            Ok(msg) => msg,
                            Err(_) => break,
                        };

                        if sink.send(msg).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        warn!("Client is lagging. Skipped {} logs.", skipped);
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        trace!("Log Reader Died");
                        break;
                    }
                }
            }
            _ = sink.closed() => {
                trace!("Log Client disconnected");
                break;
            }
        }
    }
    trace!("Stopped forwarding logs");
}
