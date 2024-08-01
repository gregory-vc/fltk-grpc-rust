use fltk::{prelude::*, *};
use gui_reflect;
mod data;
use once_cell::sync::Lazy;
use prost_reflect::DescriptorPool;

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

    let obj = data::get();
    _ = gui_reflect::tree::draw_tree(obj.p1, &DESCRIPTOR_POOL);
    _ = gui_reflect::tree::draw_tree(obj.p2, &DESCRIPTOR_POOL);
    _ = gui_reflect::tree::draw_tree(obj.s1, &DESCRIPTOR_POOL);
    _ = gui_reflect::tree::draw_tree(obj.st1, &DESCRIPTOR_POOL);

    col.end();
    scroll.end();
    scroll.resizable(&col);
    row.fixed(&scroll, 400);
    row.end();
    win.end();
    win.show_with_env_args();
    col.resize(scroll.x(), scroll.y(), scroll.w(), scroll.h());
    a.run().unwrap();
}
