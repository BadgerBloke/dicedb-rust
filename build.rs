// build.rs
// This build script only runs when the "regenerate-protos" feature is enabled
// Normal users won't need protoc or .proto files since we include pre-generated code

#[cfg(feature = "regenerate-protos")]
fn main() {
    use std::fs;

    // Create output directory
    let out_dir = "src/wire";
    fs::create_dir_all(out_dir).unwrap();

    // List of proto files (without paths/extensions)
    let protos = ["cmd", "res"];

    // Generate each proto file separately
    for proto in &protos {
        let mut config = prost_build::Config::new();
        config.out_dir(out_dir);

        // Compile single proto file
        config
            .compile_protos(&[format!("protos/{}.proto", proto)], &["protos"])
            .unwrap_or_else(|e| panic!("Failed to compile {}: {}", proto, e));
    }

    println!("Proto files regenerated in {}", out_dir);
    println!("Don't forget to commit the generated files!");
}

#[cfg(not(feature = "regenerate-protos"))]
fn main() {
    // Nothing to do - we use pre-generated files
}
