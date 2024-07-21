use solar_system_info::Class;
use solar_system_info::Planet;
use solar_system_info::Star;
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

    println!("start->>>>>>>>>>>>>>>>>>");

    for (k, v) in message.fields() {
        
        if !k.is_list() {
            println!("> {}, {}, {:?}", k.full_name(), v, k.kind());
        } else {
            println!("-> {}, {}, {:?}", k.full_name(), v, k.kind());

            if let Some(v11) = v.as_list() {
                for k11 in v11.iter() {
                    if let Some(k12) = k11.as_message() {
                        for (k13, v13) in k12.fields() {
                            if !k13.is_list() {
                                println!("-> {}, {}, {:?}", k13.full_name(), v13, k13.kind());
                            } else {
                                println!("->> {}, {}, {:?}", k13.full_name(), v13, k13.kind());
                            }
                        }
                    }else {
                        println!("------>> empty");
                    }
                }
            } else {
                println!("------>> empty as_list");
            }
        }
    }

    println!("end-<<<<<<<<<<<<<<<<<>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

    Ok(())
}

fn main() {

    let s1 = Satellite{
        id: 1,
        name: "Fobos".to_string(),
        first_spacecraft_landing_date: Some(Timestamp::from(SystemTime::now())),
    };

    let s2 = Satellite{
        id: 2,
        name: "Deimos".to_string(),
        first_spacecraft_landing_date: Some(Timestamp::from(SystemTime::now())),
    };

    let s3 = Satellite{
        id: 3,
        name: "Moon".to_string(),
        first_spacecraft_landing_date: Some(Timestamp::from(SystemTime::now())),
    };

    let s2_test = s2.clone();

    let p1 = Planet{
        id: 1,
        name: "Mars".to_string(),
        r#type: Type::DwarfPlanet.into(),
        mean_radius: 123.23,
        mass: 234234.34,
        satellites: vec![s1, s2],
    };

    let p2 = Planet{
        id: 2,
        name: "Earth".to_string(),
        r#type: Type::IceGiant.into(),
        mean_radius: 99.23,
        mass: 555.34,
        satellites: vec![s3],
    };

    let p1_test = p1.clone();
    let p2_test = p2.clone();

    let s1 = Star{
        id: 1,
        name: "Sun".to_string(),
        r#class: Class::B.into(),
        mass: 999999.99999,
        mean_radius: 777.7777,
        planets: vec![p1, p2]
    };
    
    _ = print_event(p1_test);
    _ = print_event(p2_test);
    _ = print_event(s2_test);
    _ = print_event(s1);

}