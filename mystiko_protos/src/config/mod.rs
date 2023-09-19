#[cfg(feature = "mystiko-config-api-v1")]
pub mod api;

#[cfg(feature = "mystiko-config-bridge-v1")]
pub mod bridge;
#[cfg(feature = "mystiko-config-contract-v1")]
pub mod contract;

#[cfg(feature = "mystiko-config-v1")]
pub mod v1;
