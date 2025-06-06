use std::time::{Duration, Instant};

use ::time::UtcOffset;
use alloy::uint;
use revm::primitives::U256;
use tracing_subscriber::fmt::time::OffsetTime;

pub const SEPARATORER: &str = "===============================================================================================";
pub const SEPARATOR: &str = "-----------------------------------------------------------------------------------------------";

pub const ETH_TRANSFER: &str = "<ETH transfer>";
pub const UNKNOWN: &str = "<Unknown>";

pub const ETHER: U256 = uint!(1_000_000_000_000_000_000_U256);
pub const GWEI: U256 = uint!(1_000_000_000_U256);
pub const GWEI_U128: u128 = 1_000_000_000_u128;
pub const GWEI_F64: f64 = 1_000_000_000_f64;

pub fn init_logs() {
    let offset = UtcOffset::from_hms(1, 0, 0).expect("should get CET offset");
    let time_format =
        time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]").unwrap();
    let timer = OffsetTime::new(offset, time_format);

    tracing_subscriber::fmt().with_timer(timer).init();
}

pub fn measure_start(label: &str) -> (String, Instant) {
    (label.to_string(), Instant::now())
}

pub fn measure_end(start: (String, Instant)) -> Duration {
    let elapsed = start.1.elapsed();
    tracing::info!("Elapsed: {:.2?} for '{}'", elapsed, start.0);
    elapsed
}

pub trait ToU64 {
    fn to_u64(&self) -> u64;
}

impl ToU64 for U256 {
    fn to_u64(&self) -> u64 {
        U256::to::<u64>(self)
    }
}

pub trait ToU128 {
    fn to_u128(&self) -> u128;
}

impl ToU128 for U256 {
    fn to_u128(&self) -> u128 {
        U256::to::<u128>(self)
    }
}

pub fn wei_to_eth(wei: U256) -> f64 {
    let wei_per_eth = ETHER;
    let wei_f64 = wei.to_string().parse::<f64>().unwrap();
    let wei_per_eth_f64 = wei_per_eth.to_string().parse::<f64>().unwrap();

    wei_f64 / wei_per_eth_f64
}
