// @generated
pub mod mydata {
    // @@protoc_insertion_point(attribute:mydata.v1)
    pub mod v1 {
        include!("mydata.v1.rs");
        // @@protoc_insertion_point(mydata.v1)
    }
}
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
pub mod spl {
    // @@protoc_insertion_point(attribute:spl.v1)
    pub mod v1 {
        include!("spl.v1.rs");
        // @@protoc_insertion_point(spl.v1)
    }
}
pub mod wormhole {
    // @@protoc_insertion_point(attribute:wormhole.v1)
    pub mod v1 {
        include!("wormhole.v1.rs");
        // @@protoc_insertion_point(wormhole.v1)
    }
}
