// @generated
pub mod sf {
    pub mod solana {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.solana.type.v1)
            pub mod v1 {
                include!("sf.solana.type.v1.rs");
                // @@protoc_insertion_point(sf.solana.type.v1)
            }
        }
    }
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod solana {
            // @@protoc_insertion_point(attribute:sf.substreams.solana.v1)
            pub mod v1 {
                include!("sf.substreams.solana.v1.rs");
                // @@protoc_insertion_point(sf.substreams.solana.v1)
            }
        }
    }
}
pub mod wormhole_bridge {
    pub mod message {
        // @@protoc_insertion_point(attribute:wormhole_bridge.message.v1)
        pub mod v1 {
            include!("wormhole_bridge.message.v1.rs");
            // @@protoc_insertion_point(wormhole_bridge.message.v1)
        }
    }
    pub mod nft {
        // @@protoc_insertion_point(attribute:wormhole_bridge.nft.v1)
        pub mod v1 {
            include!("wormhole_bridge.nft.v1.rs");
            // @@protoc_insertion_point(wormhole_bridge.nft.v1)
        }
    }
    pub mod token {
        // @@protoc_insertion_point(attribute:wormhole_bridge.token.v1)
        pub mod v1 {
            include!("wormhole_bridge.token.v1.rs");
            // @@protoc_insertion_point(wormhole_bridge.token.v1)
        }
    }
}
