pub mod id_mapping;

pub(crate) mod cp_adapter;
pub(crate) mod cp_config_provider;
pub(crate) mod ip_adapter;

pub(crate) use cp_adapter::CpAdapter;
pub(crate) use cp_config_provider::CpConfigProvider;
pub(crate) use ip_adapter::IpAdapter;

pub use id_mapping::{normalize_station_id, to_cp_id, to_ip_id};
