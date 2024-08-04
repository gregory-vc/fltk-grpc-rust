use fltk_theme::{color_themes, ColorTheme};
use fltk_theme::{ThemeType, WidgetTheme};
use gui_reflect;
use once_cell::sync::Lazy;
use prost_reflect::DescriptorPool;

mod data;

use fltk::{
    app,
    group::{Flex, Tabs},
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
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
    let obj = data::get();

    let p1_test1 = obj.p1.clone();
    let p2_test1 = obj.p2.clone();
    let s2_test1 = obj.s1.clone();
    let s11 = obj.st1.clone();

    _ = gui_reflect::print::print_proto(obj.p1);
    _ = gui_reflect::print::print_proto(obj.p2);
    _ = gui_reflect::print::print_proto(obj.s1);
    _ = gui_reflect::print::print_proto(obj.st1);

    let app: app::App = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::from_colormap(color_themes::DARK_THEME);
    theme.apply();

    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    widget_theme.apply();

    // app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(700, 450)
        .with_label("fltk grps rust")
        .center_screen();

    let mut tab = Tabs::default_fill();
    let mut grp1 = Flex::default_fill().with_label("Planet 1\t\t").row();
    _ = gui_reflect::draw::draw_proto(p1_test1, &DESCRIPTOR_POOL);
    grp1.make_resizable(false);
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

    // wind.set

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
