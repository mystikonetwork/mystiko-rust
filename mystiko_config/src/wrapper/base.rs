use serde::{Deserialize, Serialize};
use crate::raw::base::RawConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseConfig<R, A = ()>
    where
        R: RawConfig + Serialize,
{
    pub data: R,
    aux_data: Option<A>,
}

impl<R, A> BaseConfig<R, A> where
    R: RawConfig + Serialize, {
    pub fn new(data: R, aux_data: Option<A>) -> Self {
        Self {
            data,
            aux_data,
        }
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(&self.data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::asset::RawAssetConfig;
    use crate::wrapper::base::BaseConfig;

    #[test]
    fn test_new_base_config() {
        let asset = RawAssetConfig::new();
        let config: BaseConfig<RawAssetConfig> = BaseConfig::new(asset, None);
        let json_str = config.to_json_string();
        println!("{:?}", json_str);
    }
}


