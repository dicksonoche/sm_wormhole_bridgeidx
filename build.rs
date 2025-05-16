fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compiling the 3 local protos into Rust in OUT_DIR
    prost_build::Config::new()
        .out_dir(std::env::var("OUT_DIR")?)
        .compile_protos(
            &["proto/wormhole.proto", "proto/spl_events.proto", "proto/mydata.proto"],
            &["proto"],
        )?;
    Ok(())
}
