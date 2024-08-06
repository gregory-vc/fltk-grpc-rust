extern crate chrono;
use anyhow::Result;
use enums::CallbackTrigger;
use fltk::{prelude::*, *};
use fltk_calendar::calendar;
use menu::Choice;
use reflection::Schema;
use trees;
use trees::Node;
use reflection::{Member, Reflection};

struct MyTree {
    t: tree::Tree,
}

impl MyTree {
    pub fn default() -> Self {
        let mut t = tree::Tree::default().size_of_parent().center_of_parent();
        t.set_show_root(false);
        t.set_item_draw_mode(tree::TreeItemDrawMode::LabelAndWidget);
        t.set_widget_margin_left(0);
        Self { t }
    }
}

widget_extends!(MyTree, tree::Tree, t);

struct MyInput {}

impl MyInput {
    pub fn new(
        k: String,
        f_name: &str,
        n_name: &str,
        mut tr: MyTree,
    ) -> MyInput {
        let mut item = tree::TreeItem::new(&tr, n_name);

        item.draw_item_content(|item, render| {
            let x = item.label_x();
            let y = item.label_y();
            let w = item.label_w();
            let h = item.label_h();
            let txt = match item.label() {
                Some(s) => s,
                None => String::new(),
            };
            let txt_len = draw::measure(&txt, false).0;

            if render {
                if item.is_selected() {
                    draw::draw_rect_fill(x, y, w, h, enums::Color::DarkBlue);
                    draw::set_draw_color(enums::Color::White);
                } else {
                    draw::draw_rect_fill(x, y, w, h, item.label_bgcolor());
                    draw::set_draw_color(item.label_fgcolor());
                }

                draw::draw_text2(&txt, x, y, w, h, enums::Align::Left);

                if let Some(mut wid) = item.try_widget() {
                    wid.set_damage(true);
                }

                if let Some(mut wid) = item.try_widget() {
                    let wx = (x + txt_len + 20).max(x + (w - 210));
                    wid.resize(wx, wid.y(), 200, wid.h());
                }
            }
            x + txt_len
        });

        match k.as_str() {
            "string" => {
                let ipt = input::Input::default().with_size(200, 14);
                item.set_widget(&ipt);
            }

            "uint64" => {
                let ipt = input::FloatInput::default().with_size(200, 14);
                item.set_widget(&ipt);
            }

            "float" => {
                let ipt = input::FloatInput::default().with_size(200, 14);
                item.set_widget(&ipt);
            }

            "bool" => {
                let but = button::CheckButton::default()
                    .with_label("enabled")
                    .with_size(200, 14);
                item.set_widget(&but);
            }

            "google.protobuf.Timestamp" => {
                let mut ipt: input::Input = input::Input::default();

                ipt.set_trigger(CallbackTrigger::EnterKeyAlways);

                let mut ipt2 = ipt.clone();

                ipt.set_callback(move |_| {
                    let cal = calendar::Calendar::default();
                    let date = cal.get_date();
                    if let Some(date) = date {
                        ipt2.set_value(date.to_string().as_str());
                    }
                });

                item.set_widget(&ipt);
            }
            _ => {
                let mut chce = Choice::default();
                // for v55 in en.values() {
                //     chce.add_choice(v55.name());
                // }
                item.set_widget(&chce);
            }
        }

        tr.add_item(&format!("{}/{}", f_name, n_name), &item);

        Self {}
    }
}

pub fn draw_tree(event: Schema) -> Result<()> {
    let mut t = MyTree::default();
    schema_to_tree(event.root(), t, "".to_string());
    // t.add(message.descriptor().full_name());

    // for k in dp.all_messages() {
    //     if message.descriptor().full_name() == k.full_name() {
    //         for k2 in k.fields() {
    //             let v = message.get_field(&k2);
    //             draw(
    //                 message.descriptor().full_name(),
    //                 MyTree { t: t.clone() },
    //                 dp,
    //                 k2,
    //                 v.borrow(),
    //             );
    //         }
    //     }
    // }

    std::result::Result::Ok(())
}

fn schema_to_tree(node: &Node<Member>, mut tr: MyTree, root: String) {
    match node.data {
        Member::Field(ref field) => {
            if field.ty == reflection::Type::Enum {
                // format!(
                //     "{0}type: {1:?},\n{0}name: {2:?},\n{0}cases: {{\n{3}{0}}}",
                //     " ".repeat(level * 4),
                //     &field.tyname.clone().unwrap_or_default(),
                //     field.id,
                //     members_to_string(node, level)
                // )
                let nn = format!("{}/{}: {}", root, field.id, &field.tyname.clone().unwrap_or_default());
                tr.add(nn.as_str());
                members_to_tree(node, tr, nn);
            } else {
                let nn = format!("{}/{}: {}", root, field.id, &field.tyname.clone().unwrap_or_default());
                tr.add(nn.as_str());
                members_to_tree(node, tr, nn);
                // format!(
                //     "{0}type: {1:?},\n{0}name: {2:?},{3}",
                //     " ".repeat(level * 4),
                //     &field.tyname.clone().unwrap_or_default(),
                //     field.id,
                //     members_to_string(node, level)
                // )
            }
        }
        Member::Variant(ref variant) => {
            let nn = format!("{}/{}", root, variant.id,);
            tr.add(nn.as_str());
            members_to_tree(node, tr, nn);
            // format!(
                //     "{0}{1} => {{\n    {0}type: \"enum_val\",\n    {0}name: {2:?},{3}{0}}}",
                //     " ".repeat(level * 4),
                //     nth,
                //     variant.id,
                //     members_to_string(node, level + 1)
                // ),
        }
    }
}

fn members_to_tree(node: &Node<Member>, mut tr: MyTree, root: String) {
    for child in node.iter() {
        // let ntr = tr.clone();
        schema_to_tree(child, MyTree { t: tr.clone() }, root.clone());
        // s.push_str(&(schema_to_string(child, nth, level + 1) + &"\n"));
        // nth += 1;
    }
    // if nth == 0 {
    //     String::new()
    // } else {
    //     if let Member::Field(ref field) = node.data {
    //         if field.ty == reflection::Type::Enum {
    //             return s;
    //         }
    //     }
    //     format!("\n{0}fields: [\n{1}{0}]\n", " ".repeat(level * 4), s)
    // }
}

fn draw(f_name: &str, mut tr: MyTree) {
    // if !k.is_list() {
    //     let n_name = k.name();
    //     let nn = format!("{:?}", k.kind());
    //     let _ = MyInput::new(v, nn, dp, f_name, n_name, tr);
    // } else {
    //     let n_name = k.name();
    //     tr.add(&format!("{}/{}", f_name, n_name));

    //     if let Some(v11) = v.as_list() {
    //         if v11.len() > 0 {
    //             let mut i = 0;
    //             for k11 in v11.iter() {
    //                 i += 1;
    //                 if let Some(k12) = k11.as_message() {
    //                     for k in dp.all_messages() {
    //                         if k12.descriptor().full_name() == k.full_name() {
    //                             let next_node =
    //                                 &format!("{}/{}/{} {}", f_name, n_name, k.name(), i);
    //                             tr.add(&format!("{}/{}/{} {}", f_name, n_name, k.name(), i));

    //                             for k2 in k.fields() {
    //                                 let v = k12.get_field(&k2);
    //                                 draw(next_node, MyTree { t: tr.t.clone() }, dp, k2, v.borrow());
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
