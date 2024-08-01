use prost_types::Timestamp;
use solar_system_info::Class;
use solar_system_info::Planet;
use solar_system_info::Satellite;
use solar_system_info::Star;
use solar_system_info::Type;
use std::time::SystemTime;

pub mod solar_system_info {
    tonic::include_proto!("solar_system_info");
}

pub struct Objects {
    pub p1: Planet,
    pub p2: Planet,
    pub s1: Satellite,
    pub st1: Star,
}

pub fn get() -> Objects {
    let s1 = Satellite {
        id: 1,
        name: "Fobos".to_string(),
        first_spacecraft_landing_date: Some(Timestamp::from(SystemTime::now())),
    };

    let s2 = Satellite {
        id: 2,
        name: "Deimos".to_string(),
        first_spacecraft_landing_date: Some(Timestamp::from(SystemTime::now())),
    };

    let s3 = Satellite {
        id: 3,
        name: "Moon".to_string(),
        first_spacecraft_landing_date: Some(Timestamp::from(SystemTime::now())),
    };

    let p1 = Planet {
        id: 1,
        name: "Mars".to_string(),
        r#type: Type::DwarfPlanet.into(),
        mean_radius: 123.23,
        mass: 234234.34,
        satellites: vec![s1.clone(), s2.clone()],
        is_human: false,
    };

    let p2 = Planet {
        id: 2,
        name: "Earth".to_string(),
        r#type: Type::IceGiant.into(),
        mean_radius: 99.23,
        mass: 555.34,
        satellites: vec![s3.clone()],
        is_human: true,
    };

    let st1 = Star {
        id: 1,
        name: "Sun".to_string(),
        r#class: Class::B.into(),
        mass: 999999.99999,
        mean_radius: 777.7777,
        planets: vec![p1.clone(), p2.clone()],
    };

    return Objects { p1, p2, s1, st1 };
}
