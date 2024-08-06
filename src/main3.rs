use fltk::{
    prelude::*,
    *,
};
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
    let a = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .center_screen();

    let obj = data::get();
    _ = gui_reflect::tree::draw_tree(obj.st1, &DESCRIPTOR_POOL);

    win.end();
    win.make_resizable(true);
    win.show();

    a.run().unwrap();
}
