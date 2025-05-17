fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .compile_protos(
            &[
                "proto/wormhole_bridge/message.proto",
                "proto/wormhole_bridge/token.proto",
                "proto/wormhole_bridge/nft.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
