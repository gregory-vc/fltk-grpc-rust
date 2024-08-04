extern crate reflection_derive;
use reflection_derive::Reflection;

extern crate reflection;
use reflection::{Member, Reflection};

use trees;
use trees::Node;

fn main() {
    #[derive(Reflection)]
    enum Foo {
        Bar { a: u64 },
        Bla { b: u64 },
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

    let out = schema_to_string(Foo::schemata().root(), 0, 0);

    println!("{}", out)
}
