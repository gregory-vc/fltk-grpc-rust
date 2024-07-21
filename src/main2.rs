use prost_reflect::FieldDescriptor;
use prost_reflect::Value;
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

fn print_message(del: &String, k: FieldDescriptor, v: &Value) {
    let next_del = del.to_owned()+">";
    if !k.is_list() {
        println!("{} {}, {}, {:?}", del, k.full_name(), v, k.kind());
    } else {
        println!("{} {}, {}, {:?}", next_del, k.full_name(), v, k.kind());

        if let Some(v11) = v.as_list() {
            for k11 in v11.iter() {
                if let Some(k12) = k11.as_message() {
                    for (k13, v13) in k12.fields() {
                        print_message(&next_del, k13, v13);
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

fn print_proto(event: impl ReflectMessage) -> Result<()> {
    let message: prost_reflect::DynamicMessage = format::proto2dynamic(event)?;

    println!("start__--------------------------------------------------->");

    for (k, v) in message.fields() {
        print_message(&">".to_string(), k, v);
    }

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
    
    _ = print_proto(p1_test);
    _ = print_proto(p2_test);
    _ = print_proto(s2_test);
    _ = print_proto(s1);

}