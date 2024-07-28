use solar_system_info::Class;
use solar_system_info::Planet;
use solar_system_info::Star;
use solar_system_info::Type;
use solar_system_info::Satellite;
use prost_types::Timestamp;
use std::time::SystemTime;
use once_cell::sync::Lazy;
use prost_reflect::DescriptorPool;
use gui_reflect;

use fltk::{
    app,
    window::Window,
    group::{Flex, Tabs},
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
};


pub mod solar_system_info {
    tonic::include_proto!("solar_system_info");
}

pub static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    DescriptorPool::decode(
        include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
    )
    .unwrap()
});

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

    let p1_test1 = p1_test.clone();
    let p2_test1 = p2_test.clone();
    let s2_test1 = s2_test.clone();
    let s11 = s1.clone();
    
    _ = gui_reflect::print::print_proto(p1_test);
    _ = gui_reflect::print::print_proto(p2_test);
    _ = gui_reflect::print::print_proto(s2_test);
    _ = gui_reflect::print::print_proto(s1);

    let app: app::App = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(700, 450)
        .with_label("fltk grps rust")
        .center_screen();

    let mut tab = Tabs::default_fill();

    let grp1 = Flex::default_fill().with_label("Planet 1\t\t").row();
    _ = gui_reflect::draw::draw_proto(p1_test1, &DESCRIPTOR_POOL);
    grp1.end();

    let grp2 = Flex::default_fill().with_label("Planet 2\t\t").row();
    _ = gui_reflect::draw::draw_proto(p2_test1, &DESCRIPTOR_POOL);
    grp2.end();

    let grp2 = Flex::default_fill().with_label("Satellite\t\t").row();
    _ = gui_reflect::draw::draw_proto(s2_test1, &DESCRIPTOR_POOL);
    grp2.end();

    let grp2 = Flex::default_fill().with_label("Star\t\t").row();
    _ = gui_reflect::draw::draw_proto(s11, &DESCRIPTOR_POOL);
    grp2.end();

    tab.end();
    tab.auto_layout();


    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}