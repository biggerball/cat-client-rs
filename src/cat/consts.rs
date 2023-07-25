pub const ROUTER_PATH: &str = "/cat/s/router";

pub const LOCALHOST: &str = "127.0.0.1";

pub const DEFAULT_APP_KEY: &str = "cat";
pub const DEFAULT_HOSTNAME: &str = "GoUnknownHost";
pub const DEFAULT_ENV: &str = "dev";
pub const DEFAULT_IP: &str = "127.0.0.1";
pub const DEFAULT_IP_HEX: &str = "7f000001";
pub const DEFAULT_XML_FILE: &str = "/data/appdatas/cat/rust-client.xml";
pub const DEFAULT_LOG_DIR: &str = "/data/applogs/cat";
pub const TMP_LOG_DIR: &str = "/tmp";


pub const TYPE_SYSTEM: &str = "System";
pub const NAME_REBOOT: &str = "Reboot";
pub const NAME_TRANSACTION_AGGREGATOR: &str = "TransactionAggregator";
pub const NAME_EVENT_AGGREGATOR: &str = "EventAggregator";
pub const NAME_METRIC_AGGREGATOR: &str = "MetricAggregator";

pub const PROPERTY_SAMPLE: &str  = "sample";
pub const PROPERTY_ROUTERS: &str  = "routers";
pub const PROPERTY_BLOCK: &str  = "block";

pub const HIGH_PRIORITY_QUEUE_SIZE: usize = 1000;
pub const NORMAL_PRIORITY_QUEUE_SIZE: usize = 5000;

#[derive(Clone)]
pub enum Signal {
    SignalResetConnection,
    SignalShutdown,
    SignalManagerExit,
    SignalSenderExit,
    SignalMonitorExit,
    SignalRouterExit,
    SignalTransactionAggregatorExit,
    SignalEventAggregatorExit,
    SignalMetricAggregatorExit,
}