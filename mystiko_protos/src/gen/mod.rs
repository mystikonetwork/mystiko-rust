// @generated
pub mod mystiko {
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
    pub mod sequencer {
        #[cfg(feature = "mystiko-sequencer-v1")]
        // @@protoc_insertion_point(attribute:mystiko.sequencer.v1)
        pub mod v1 {
            include!("mystiko.sequencer.v1.rs");
            // @@protoc_insertion_point(mystiko.sequencer.v1)
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
}
