// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WormholeMessage {
    #[prost(string, tag="1")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub sequence_number: u32,
    #[prost(uint32, tag="3")]
    pub nonce_value: u32,
    #[prost(string, tag="4")]
    pub source_address: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub consistency_level: u32,
    #[prost(bytes="vec", tag="6")]
    pub message_payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub block_time: u64,
    #[prost(uint64, tag="8")]
    pub block_height: u64,
    #[prost(uint32, tag="9")]
    pub source_chain: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WormholeMessages {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<WormholeMessage>,
}
// @@protoc_insertion_point(module)
