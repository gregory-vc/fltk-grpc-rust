use fltk::{prelude::*, *};
use gui_reflect;
use once_cell::sync::Lazy;
use prost_reflect::DescriptorPool;
use prost_types::Timestamp;
use solar_system_info::Class;
use solar_system_info::Planet;
use solar_system_info::Satellite;
use solar_system_info::Star;
use solar_system_info::Type;
use std::time::SystemTime;

// static COFACTOR: utils::oncelock::Lazy<i32> =
//     utils::oncelock::Lazy::new(|| (app::font_size() as f64 * 2.0) as i32);

// fn prep_tree(t: &mut tree::Tree) {
//     if let Some(root) = t.next(&t.first().unwrap()) {
//         if root.is_open() {
//             let elems = root.children();
//             t.resize(t.x(), t.y(), t.w(), (elems + 1) * *COFACTOR);
//         } else {
//             t.set_scrollbar_size(0);
//             t.resize(t.x(), t.y(), t.w(), *COFACTOR);
//         }
//     } else {
//         t.resize(t.x(), t.y(), t.w(), *COFACTOR);
//     }
//     app::redraw();
// }

// struct MyTree {
//     t: tree::Tree,
// }

// impl MyTree {
//     pub fn default() -> Self {
//         let mut t = tree::Tree::default();
//         t.set_show_root(false);
//         t.set_callback(prep_tree);
//         Self { t }
//     }
//     pub fn end(&mut self) {
//         prep_tree(&mut self.t);
//     }
// }

// widget_extends!(MyTree, tree::Tree, t);

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
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(800, 600);
    let mut row = group::Flex::default_fill().row();
    let mut scroll = group::Scroll::default();
    let mut col = group::Pack::default().with_type(group::PackType::Vertical);

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

    let s2_test = s2.clone();

    let p1 = Planet {
        id: 1,
        name: "Mars".to_string(),
        r#type: Type::DwarfPlanet.into(),
        mean_radius: 123.23,
        mass: 234234.34,
        satellites: vec![s1, s2],
        is_human: false,
    };

    let p2 = Planet {
        id: 2,
        name: "Earth".to_string(),
        r#type: Type::IceGiant.into(),
        mean_radius: 99.23,
        mass: 555.34,
        satellites: vec![s3],
        is_human: true,
    };

    let p1_test = p1.clone();
    let p2_test = p2.clone();

    let s1 = Star {
        id: 1,
        name: "Sun".to_string(),
        r#class: Class::B.into(),
        mass: 999999.99999,
        mean_radius: 777.7777,
        planets: vec![p1, p2],
    };

    // let p1_test1 = p1_test.clone();
    // let p2_test1 = p2_test.clone();
    // let s2_test1 = s2_test.clone();
    // let s11 = s1.clone();

    _ = gui_reflect::tree::draw_tree(s1, &DESCRIPTOR_POOL);
    _ = gui_reflect::tree::draw_tree(p1_test, &DESCRIPTOR_POOL);
    _ = gui_reflect::tree::draw_tree(p2_test, &DESCRIPTOR_POOL);
    _ = gui_reflect::tree::draw_tree(s2_test, &DESCRIPTOR_POOL);

    // let mut t = MyTree::default();
    // t.add("Source Control");
    // for i in 1..4 {
    //     t.add(&format!("Source Control/Repo {}", i));
    //     t.add_item(path, item)
    // }
    // t.end();
    // let mut t = MyTree::default();
    // t.add("Commits");
    // for i in 1..30 {
    //     t.add(&format!("Commits/Commit {}", i));
    // }
    // t.end();
    // let mut t = MyTree::default();
    // t.add("Branches");
    // t.add("Branches/main");
    // t.add("Branches/dev");
    // t.end();

    col.end();
    scroll.end();
    scroll.resizable(&col);
    row.fixed(&scroll, 200);
    row.end();
    win.end();
    win.show_with_env_args();
    col.resize(scroll.x(), scroll.y(), scroll.w(), scroll.h());
    a.run().unwrap();
}
