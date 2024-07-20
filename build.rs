// fn main() -> Result<(), Box<dyn std::error::Error>> {

//     prost_reflect_build::Builder::new()
//     .compile_protos(&["src/proto/solar-system-info.proto"], &["src/proto"])
//     .unwrap();

//     tonic_build::compile_protos("src/proto/hello.proto")?;
//     tonic_build::compile_protos("src/proto/solar-system-info.proto")?;


//     Ok(())
// }

use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .configure(
            &mut config,
            &[
                "src/proto/hello.proto",
                "src/proto/solar-system-info.proto",
            ],
            &["src/proto/"],
        )?;
    tonic_build::configure().compile_with_config(
        config,
        &[
            "src/proto/hello.proto",
            "src/proto/solar-system-info.proto",
        ],
        &["src/proto/"],
    )?;
    Ok(())
}