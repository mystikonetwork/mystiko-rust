use crate::core::v1::{PackerChecksum, PackerCompression};

impl From<&mystiko_types::PackerChecksum> for PackerChecksum {
    fn from(value: &mystiko_types::PackerChecksum) -> Self {
        match value {
            mystiko_types::PackerChecksum::Sha512 => PackerChecksum::Sha512,
        }
    }
}

impl From<&mystiko_types::PackerCompression> for PackerCompression {
    fn from(value: &mystiko_types::PackerCompression) -> Self {
        match value {
            mystiko_types::PackerCompression::Zstd => PackerCompression::Zstd,
        }
    }
}
