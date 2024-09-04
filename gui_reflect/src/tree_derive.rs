extern crate chrono;
use anyhow::Result;
use enums::CallbackTrigger;
use fltk::{prelude::*, *};
use fltk_calendar::calendar;
use menu::Choice;
use reflection::Member;
use reflection::Schema;
use trees;
use trees::Node;

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
        node: &Node<Member>,
        fld: &reflection::Field,
        f_name: &str,
        mut tr: MyTree,
    ) -> MyInput {
        let n_name: String;
        if fld.id == "_" {
            n_name = format!("{}: {}", "1", &fld.tyname.clone().unwrap_or_default());
        } else {
            n_name = format!("{}: {}", fld.id, &fld.tyname.clone().unwrap_or_default());
        }

        let ty = fld.ty;

        let mut item = tree::TreeItem::new(&tr, n_name.as_str());

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

        match ty {
            reflection::Type::U64 => {
                let ipt = input::IntInput::default().with_size(200, 14);
                item.set_widget(&ipt);
            }
            reflection::Type::F64 => {
                let ipt = input::FloatInput::default().with_size(200, 14);
                item.set_widget(&ipt);
            }
            reflection::Type::Bool => {
                let but = button::CheckButton::default().with_size(200, 14);
                item.set_widget(&but);
            }
            reflection::Type::String => {
                let ipt = input::Input::default().with_size(200, 14);
                item.set_widget(&ipt);
            }
            reflection::Type::Enum => {
                if &fld.tyname.clone().unwrap_or_default() == "TimeTime" {
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
                } else {
                    let mut chce: Choice = Choice::default();
                    for child in node.iter() {
                        match child.data {
                            Member::Field(ref _field) => {}
                            Member::Variant(ref variant) => {
                                chce.add_choice(variant.id);
                            }
                        }
                    }
                    item.set_widget(&chce);
                }
            }
            _ => {}
        }

        tr.add_item(&format!("{}/{}", f_name, n_name), &item);

        Self {}
    }
}

pub fn draw_tree(event: Schema) -> Result<()> {
    let t = MyTree::default();
    schema_to_tree(event.root(), t, "".to_string());
    std::result::Result::Ok(())
}

fn schema_to_tree(node: &Node<Member>, mut tr: MyTree, root: String) {
    match node.data {
        Member::Field(ref field) => {
            if field.ty == reflection::Type::Enum {
                let _ = MyInput::new(node, field, root.as_str(), tr);
            } else {
                let nn;
                if field.id != "_" {
                    let _ = MyInput::new(node, field, root.as_str(), MyTree { t: tr.clone() });
                    nn = format!(
                        "{}/{}: {}",
                        root,
                        field.id,
                        &field.tyname.clone().unwrap_or_default()
                    )
                } else {
                    nn = format!(
                        "{}/{}: {}",
                        root,
                        1,
                        &field.tyname.clone().unwrap_or_default()
                    )
                }
                members_to_tree(node, tr, nn);
            }
        }
        Member::Variant(ref variant) => {
            let nn = format!("{}/{}", root, variant.id,);
            tr.add(nn.as_str());
            members_to_tree(node, tr, nn);
        }
    }
}

fn members_to_tree(node: &Node<Member>, tr: MyTree, root: String) {
    for child in node.iter() {
        schema_to_tree(child, MyTree { t: tr.clone() }, root.clone());
    }
}
