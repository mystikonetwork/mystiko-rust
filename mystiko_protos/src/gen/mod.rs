// @generated
pub mod mystiko {
    pub mod api {
        pub mod config {
            #[cfg(feature = "mystiko-api-config-v1")]
            // @@protoc_insertion_point(attribute:mystiko.api.config.v1)
            pub mod v1 {
                include!("mystiko.api.config.v1.rs");
                // @@protoc_insertion_point(mystiko.api.config.v1)
            }
        }
        pub mod handler {
            #[cfg(feature = "mystiko-api-handler-v1")]
            // @@protoc_insertion_point(attribute:mystiko.api.handler.v1)
            pub mod v1 {
                include!("mystiko.api.handler.v1.rs");
                // @@protoc_insertion_point(mystiko.api.handler.v1)
            }
        }
        pub mod scanner {
            #[cfg(feature = "mystiko-api-scanner-v1")]
            // @@protoc_insertion_point(attribute:mystiko.api.scanner.v1)
            pub mod v1 {
                include!("mystiko.api.scanner.v1.rs");
                // @@protoc_insertion_point(mystiko.api.scanner.v1)
            }
        }
        pub mod synchronizer {
            #[cfg(feature = "mystiko-api-synchronizer-v1")]
            // @@protoc_insertion_point(attribute:mystiko.api.synchronizer.v1)
            pub mod v1 {
                include!("mystiko.api.synchronizer.v1.rs");
                // @@protoc_insertion_point(mystiko.api.synchronizer.v1)
            }
        }
        #[cfg(feature = "mystiko-api-v1")]
        // @@protoc_insertion_point(attribute:mystiko.api.v1)
        pub mod v1 {
            include!("mystiko.api.v1.rs");
            // @@protoc_insertion_point(mystiko.api.v1)
        }
    }
    pub mod common {
        #[cfg(feature = "mystiko-common-v1")]
        // @@protoc_insertion_point(attribute:mystiko.common.v1)
        pub mod v1 {
            include!("mystiko.common.v1.rs");
            // @@protoc_insertion_point(mystiko.common.v1)
        }
    }
    pub mod config {
        pub mod bridge {
            #[cfg(feature = "mystiko-config-bridge-v1")]
            // @@protoc_insertion_point(attribute:mystiko.config.bridge.v1)
            pub mod v1 {
                include!("mystiko.config.bridge.v1.rs");
                // @@protoc_insertion_point(mystiko.config.bridge.v1)
            }
        }
        pub mod contract {
            #[cfg(feature = "mystiko-config-contract-v1")]
            // @@protoc_insertion_point(attribute:mystiko.config.contract.v1)
            pub mod v1 {
                include!("mystiko.config.contract.v1.rs");
                // @@protoc_insertion_point(mystiko.config.contract.v1)
            }
        }
        #[cfg(feature = "mystiko-config-v1")]
        // @@protoc_insertion_point(attribute:mystiko.config.v1)
        pub mod v1 {
            include!("mystiko.config.v1.rs");
            // @@protoc_insertion_point(mystiko.config.v1)
        }
    }
    pub mod core {
        pub mod document {
            #[cfg(feature = "mystiko-core-document-v1")]
            // @@protoc_insertion_point(attribute:mystiko.core.document.v1)
            pub mod v1 {
                include!("mystiko.core.document.v1.rs");
                // @@protoc_insertion_point(mystiko.core.document.v1)
            }
        }
        pub mod handler {
            #[cfg(feature = "mystiko-core-handler-v1")]
            // @@protoc_insertion_point(attribute:mystiko.core.handler.v1)
            pub mod v1 {
                include!("mystiko.core.handler.v1.rs");
                // @@protoc_insertion_point(mystiko.core.handler.v1)
            }
        }
        pub mod scanner {
            #[cfg(feature = "mystiko-core-scanner-v1")]
            // @@protoc_insertion_point(attribute:mystiko.core.scanner.v1)
            pub mod v1 {
                include!("mystiko.core.scanner.v1.rs");
                // @@protoc_insertion_point(mystiko.core.scanner.v1)
            }
        }
        pub mod synchronizer {
            #[cfg(feature = "mystiko-core-synchronizer-v1")]
            // @@protoc_insertion_point(attribute:mystiko.core.synchronizer.v1)
            pub mod v1 {
                include!("mystiko.core.synchronizer.v1.rs");
                // @@protoc_insertion_point(mystiko.core.synchronizer.v1)
            }
        }
        #[cfg(feature = "mystiko-core-v1")]
        // @@protoc_insertion_point(attribute:mystiko.core.v1)
        pub mod v1 {
            include!("mystiko.core.v1.rs");
            // @@protoc_insertion_point(mystiko.core.v1)
        }
    }
    pub mod data {
        #[cfg(feature = "mystiko-data-v1")]
        // @@protoc_insertion_point(attribute:mystiko.data.v1)
        pub mod v1 {
            include!("mystiko.data.v1.rs");
            // @@protoc_insertion_point(mystiko.data.v1)
        }
    }
    pub mod loader {
        #[cfg(feature = "mystiko-loader-v1")]
        // @@protoc_insertion_point(attribute:mystiko.loader.v1)
        pub mod v1 {
            include!("mystiko.loader.v1.rs");
            // @@protoc_insertion_point(mystiko.loader.v1)
        }
    }
    pub mod relayer {
        #[cfg(feature = "mystiko-relayer-v1")]
        // @@protoc_insertion_point(attribute:mystiko.relayer.v1)
        pub mod v1 {
            include!("mystiko.relayer.v1.rs");
            // @@protoc_insertion_point(mystiko.relayer.v1)
        }
    }
    pub mod screening {
        #[cfg(feature = "mystiko-screening-v1")]
        // @@protoc_insertion_point(attribute:mystiko.screening.v1)
        pub mod v1 {
            include!("mystiko.screening.v1.rs");
            // @@protoc_insertion_point(mystiko.screening.v1)
        }
    }
    pub mod sequencer {
        #[cfg(feature = "mystiko-sequencer-v1")]
        // @@protoc_insertion_point(attribute:mystiko.sequencer.v1)
        pub mod v1 {
            include!("mystiko.sequencer.v1.rs");
            // @@protoc_insertion_point(mystiko.sequencer.v1)
        }
    }
    pub mod service {
        #[cfg(feature = "mystiko-service-v1")]
        // @@protoc_insertion_point(attribute:mystiko.service.v1)
        pub mod v1 {
            include!("mystiko.service.v1.rs");
            // @@protoc_insertion_point(mystiko.service.v1)
        }
    }
    pub mod storage {
        #[cfg(feature = "mystiko-storage-v1")]
        // @@protoc_insertion_point(attribute:mystiko.storage.v1)
        pub mod v1 {
            include!("mystiko.storage.v1.rs");
            // @@protoc_insertion_point(mystiko.storage.v1)
        }
    }
    pub mod testing {
        #[cfg(feature = "mystiko-testing-v1")]
        // @@protoc_insertion_point(attribute:mystiko.testing.v1)
        pub mod v1 {
            include!("mystiko.testing.v1.rs");
            // @@protoc_insertion_point(mystiko.testing.v1)
        }
    }
}
