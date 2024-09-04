extern crate reflection;
extern crate reflection_derive;
use fltk::{prelude::*, *};
use reflection::Reflection;
use reflection_derive::Reflection;

fn main() {
    #[derive(Reflection)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Reflection)]
    struct Option {
        option_name: String,
    }

    #[derive(Reflection)]
    struct Username {
        name: String,
        source: String,
        active: bool,
        option: Vec<Option>,
    }

    #[derive(Reflection)]
    enum TimeTime {
        Tm(String),
    }

    #[derive(Reflection)]
    struct IP {
        Val: String,
        Type: IpAddrKind,
    }

    #[derive(Reflection)]
    struct User {
        active: bool,
        username: Vec<Username>,
        email: String,
        ips: Vec<IP>,
        sign_in_count: u64,
        register_time: TimeTime,
    }

    _ = gui_reflect::print_derive::print_tree(User::schemata());

    let a = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .center_screen();

    _ = gui_reflect::tree_derive::draw_tree(User::schemata());

    win.end();
    win.make_resizable(true);
    win.show();

    a.run().unwrap();
}
