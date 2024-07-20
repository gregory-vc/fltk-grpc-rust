use solar_system_info::Planet;
use solar_system_info::Type;
use solar_system_info::Satellite;
use prost_types::Timestamp;
use std::time::SystemTime;
use once_cell::sync::Lazy;
use prost_reflect::DescriptorPool;
use prost_reflect::ReflectMessage;
use anyhow::Result;

pub mod format;

pub mod solar_system_info {
    tonic::include_proto!("solar_system_info");
}

pub static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    DescriptorPool::decode(
        include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
    )
    .unwrap()
});

fn print_event(event: impl ReflectMessage) -> Result<()> {
    let message: prost_reflect::DynamicMessage = format::proto2dynamic(event)?;
    // println!("planet: {:?}", message);


    for (k, v) in message.fields() {
        println!("> {}, {}, {}", k.full_name(), v, k.is_list());
    }
    
    // let mut value = serde_json::to_value(&message)?;
    // println!("{}", serde_json::to_string(&value)?);

    // let mut map = format::proto2kv(event)?;
    // println!("{}", format::kv2line(map),);

    Ok(())
}

fn main() {

    let p = Planet{
        id: 1,
        name: "Mars".to_string(),
        r#type: Type::TerrestrialPlanet.into(),
        mean_radius: 123.23,
        mass: 234234.34,
        satellites: vec![Satellite{
            id: 1,
            name: "Fobos".to_string(),
            first_spacecraft_landing_date: Some(Timestamp::from(SystemTime::now())),
        }],
    };

    println!("planet: {:?}", p);

    _ = print_event(p);

}