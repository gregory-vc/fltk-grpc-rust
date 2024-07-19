use solar_system_info::Planet;
use solar_system_info::Type;
use solar_system_info::Satellite;
use prost_types::Timestamp;
use std::time::SystemTime;

pub mod solar_system_info {
    tonic::include_proto!("solar_system_info");
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

    println!("Intercepting request: {:?}", p);
}