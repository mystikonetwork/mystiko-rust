use crate::errors::EtherScanError;
use crate::response::Result;

pub const DEFAULT_MAX_REQUESTS_PER_SECOND: u128 = 5;
pub const DEFAULT_PAGE_OFFSET: u64 = 1000;
pub const MAX_OFFSET: u64 = 10000;
pub const DEFAULT_URL_PREFIX: &str = "/api";

const DEFAULT_MAINNET_ETHER_API_BASE_URL: &str = "https://api.etherscan.io";
const DEFAULT_MAINNET_BSC_API_BASE_URL: &str = "https://api.bscscan.com";
const DEFAULT_MAINNET_POLYGON_API_BASE_URL: &str = "https://api.polygonscan.com";
const DEFAULT_MAINNET_AVALANCHE_API_BASE_URL: &str = "https://api.snowtrace.io";
const DEFAULT_MAINNET_FANTOM_API_BASE_URL: &str = "https://api.ftmscan.com";
const DEFAULT_MAINNET_MOONBEAM_API_BASE_URL: &str = "https://api-moonbeam.moonscan.io";
const DEFAULT_TESTNET_GOERLI_API_BASE_URL: &str = "https://api-goerli.etherscan.io";
const DEFAULT_TESTNET_BSC_API_BASE_URL: &str = "https://api-testnet.bscscan.com";
const DEFAULT_TESTNET_POLYGAN_MUMBAI_API_BASE_URL: &str = "https://api-testnet.polygonscan.com";
const DEFAULT_TESTNET_AVALANCHE_FUJI_API_BASE_URL: &str = "https://api-testnet.snowtrace.io";
const DEFAULT_TESTNET_FANTOM_API_BASE_URL: &str = "https://api-testnet.ftmscan.com";
const DEFAULT_TESTNET_MOONBASE_ALPHA_API_BASE_URL: &str = "https://api-moonbase.moonscan.io";

pub fn get_default_base_url(chain_id: u64) -> Result<String> {
    match chain_id {
        1 => Ok(String::from(DEFAULT_MAINNET_ETHER_API_BASE_URL)),
        56 => Ok(String::from(DEFAULT_MAINNET_BSC_API_BASE_URL)),
        137 => Ok(String::from(DEFAULT_MAINNET_POLYGON_API_BASE_URL)),
        43114 => Ok(String::from(DEFAULT_MAINNET_AVALANCHE_API_BASE_URL)),
        250 => Ok(String::from(DEFAULT_MAINNET_FANTOM_API_BASE_URL)),
        1284 => Ok(String::from(DEFAULT_MAINNET_MOONBEAM_API_BASE_URL)),
        5 => Ok(String::from(DEFAULT_TESTNET_GOERLI_API_BASE_URL)),
        97 => Ok(String::from(DEFAULT_TESTNET_BSC_API_BASE_URL)),
        80001 => Ok(String::from(DEFAULT_TESTNET_POLYGAN_MUMBAI_API_BASE_URL)),
        43113 => Ok(String::from(DEFAULT_TESTNET_AVALANCHE_FUJI_API_BASE_URL)),
        4002 => Ok(String::from(DEFAULT_TESTNET_FANTOM_API_BASE_URL)),
        1287 => Ok(String::from(DEFAULT_TESTNET_MOONBASE_ALPHA_API_BASE_URL)),
        _ => Err(EtherScanError::UnsupportedChainIdError(chain_id)),
    }
}
