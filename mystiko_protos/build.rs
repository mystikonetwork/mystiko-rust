extern crate prost_build;
use std::fs;

fn main() {
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    let result = config.out_dir("./src/data_packer/v1/").bytes(["."]).compile_protos(
        &[
            "protos/mystiko/data_packer/v1/commitment.proto",
            "protos/mystiko/data_packer/v1/nullifier.proto",
            "protos/mystiko/data_packer/v1/packer.proto",
        ],
        &["protos"],
    );
    println!("{:?}", result);
    assert!(result.is_ok());
    let result = fs::rename(
        "src/data_packer/v1/mystiko.data_packer.v1.rs",
        "src/data_packer/v1/packed_data.rs",
    );
    assert!(result.is_ok());
}
