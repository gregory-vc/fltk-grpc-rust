extern crate reflection;
extern crate reflection_derive;
use reflection::{Member, Reflection};
use reflection_derive::Reflection;
use trees;
use trees::Node;

use fltk::{prelude::*, *};

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

    fn schema_to_string(node: &Node<Member>, nth: usize, level: usize) -> String {
        match node.data {
            Member::Field(ref field) => {
                if field.ty == reflection::Type::Enum {
                    format!(
                        "{0}type: {1:?},\n{0}name: {2:?},\n{0}cases: {{\n{3}{0}}}",
                        " ".repeat(level * 4),
                        &field.tyname.clone().unwrap_or_default(),
                        field.id,
                        members_to_string(node, level)
                    )
                } else {
                    format!(
                        "{0}type: {1:?},\n{0}name: {2:?},{3}",
                        " ".repeat(level * 4),
                        &field.tyname.clone().unwrap_or_default(),
                        field.id,
                        members_to_string(node, level)
                    )
                }
            }
            Member::Variant(ref variant) => format!(
                "{0}{1} => {{\n    {0}type: \"enum_val\",\n    {0}name: {2:?},{3}{0}}}",
                " ".repeat(level * 4),
                nth,
                variant.id,
                members_to_string(node, level + 1)
            ),
        }
    }

    fn members_to_string(node: &Node<Member>, level: usize) -> String {
        let mut s = String::new();
        let mut nth = 0usize;
        for child in node.iter() {
            s.push_str(&(schema_to_string(child, nth, level + 1) + &"\n"));
            nth += 1;
        }
        if nth == 0 {
            String::new()
        } else {
            if let Member::Field(ref field) = node.data {
                if field.ty == reflection::Type::Enum {
                    return s;
                }
            }
            format!("\n{0}fields: [\n{1}{0}]\n", " ".repeat(level * 4), s)
        }
    }

    let out = schema_to_string(User::schemata().root(), 0, 0);

    println!("{}", out);

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
