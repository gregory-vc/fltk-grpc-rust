extern crate chrono;

use std::borrow::Borrow;

use anyhow::Ok;
use anyhow::Result;
use chrono::prelude::*;
use enums::Align;
use enums::CallbackTrigger;
use enums::Color;
use fltk::{prelude::*, *};
use fltk_calendar::calendar;
use menu::Choice;
use prost_reflect::DescriptorPool;
use prost_reflect::DynamicMessage;
use prost_reflect::FieldDescriptor;
use prost_reflect::ReflectMessage;
use prost_reflect::Value;

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

fn proto2dynamic(proto: impl ReflectMessage) -> Result<DynamicMessage> {
    Ok(DynamicMessage::decode(
        proto.descriptor(),
        proto.encode_to_vec().as_slice(),
    )?)
}

struct MyInput {}

impl MyInput {
    pub fn new(
        v: &Value,
        k: String,
        dp: &DescriptorPool,
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
                    // the widget needs to be manually positioned during each render
                    let wx = (x + txt_len + 20).max(x + (w - 210));
                    // println!("x: {}, txt_len: {}, w: {} = wx: {}", x, txt_len, w, wx);
                    wid.resize(wx, wid.y(), 200, wid.h());
                }
            }

            // this returned value has little effect in this context
            x + txt_len
        });

        match k.as_str() {
            "string" => {
                let mut ipt = input::Input::default().with_size(200, 14);
                if let Some(vl) = v.as_str() {
                    ipt.set_value(vl);
                }
                item.set_widget(&ipt);
            }

            "uint64" => {
                let mut ipt = input::FloatInput::default().with_size(200, 14);
                if let Some(vl) = v.as_u64() {
                    ipt.set_value(vl.to_string().as_str());
                }
                item.set_widget(&ipt);
            }

            "float" => {
                let mut ipt = input::FloatInput::default().with_size(200, 14);
                if let Some(vl) = v.as_f32() {
                    ipt.set_value(vl.to_string().as_str());
                }
                item.set_widget(&ipt);
            }

            "bool" => {
                let mut but = button::CheckButton::default()
                    .with_label("enabled")
                    .with_size(200, 14);
                if let Some(vl) = v.as_bool() {
                    but.set_value(vl);
                }
                item.set_widget(&but);
            }

            "google.protobuf.Timestamp" => {
                let mut ipt: input::Input = input::Input::default();

                if let Some(k55) = v.as_message() {
                    if let Some(s55) = k55.get_field_by_name("seconds").as_ref() {
                        if let Some(vl) = s55.as_i64() {
                            if let Some(dt56) = DateTime::from_timestamp(vl, 0) {
                                ipt.set_value(dt56.format("%Y-%m-%d").to_string().as_str());
                            }
                        }
                    }
                }

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
                if let Some(vv1) = v.as_enum_number() {
                    if let Some(en) = dp.get_enum_by_name(k.as_str()) {
                        let mut chce = Choice::default();
                        for v55 in en.values() {
                            chce.add_choice(v55.name());
                        }
                        chce.set_value(vv1);
                        item.set_widget(&chce);
                    }
                }
            }
        }

        tr.add_item(&format!("{}/{}", f_name, n_name), &item);

        Self {}
    }
}

pub fn draw_tree(event: impl ReflectMessage, dp: &DescriptorPool) -> Result<()> {
    let message: prost_reflect::DynamicMessage = proto2dynamic(event)?;
    // let mut col = group::Flex::default_fill().column();
    // col.set_margin(10);

    let mut t = MyTree::default();
    t.add(message.descriptor().full_name());

    for k in dp.all_messages() {
        if message.descriptor().full_name() == k.full_name() {
            for k2 in k.fields() {
                let v = message.get_field(&k2);
                draw(
                    message.descriptor().full_name(),
                    MyTree { t: t.clone() },
                    dp,
                    k2,
                    v.borrow(),
                );
            }
        }
    }

    // t.end();
    // col.end();
    // col.set_pad(10);

    std::result::Result::Ok(())
}

fn draw(f_name: &str, mut tr: MyTree, dp: &DescriptorPool, k: FieldDescriptor, v: &Value) {
    // let mut row: group::Flex = group::Flex::default().row();
    // let next_pad = pad + 40;
    // let mut row_vec: Vec<group::Flex> = Vec::new();

    if !k.is_list() {
        let n_name = k.name();

        // tr.callback_item()
        // let name = k.full_name();
        // let _ = MyFrame::new(&name, enums::Color::DarkMagenta);
        // let nn = format!("{:?}", k.kind());
        // let _ = MyFrame::new(&nn, enums::Color::DarkMagenta);
        let nn = format!("{:?}", k.kind());
        let _ = MyInput::new(v, nn, dp, f_name, n_name, tr);

        // row.end();
        // row.set_margins(pad, 0, 0, 0);
    } else {
        let n_name = k.name();
        tr.add(&format!("{}/{}", f_name, n_name));
        // let name = k.full_name();
        // let _ = MyFrame::new(&name, enums::Color::Inactive);
        // let nn = format!("{:?}", k.kind());
        // let _ = MyFrame::new(&nn, enums::Color::Inactive);

        if let Some(v11) = v.as_list() {
            if v11.len() > 0 {
                // let mut but = button::Button::new(160, 200, 80, 40, ">");
                // row.end();
                // row.set_margins(pad, 0, 0, 0);
                let mut i = 0;
                for k11 in v11.iter() {
                    i += 1;
                    if let Some(k12) = k11.as_message() {
                        for k in dp.all_messages() {
                            if k12.descriptor().full_name() == k.full_name() {
                                let next_node =
                                    &format!("{}/{}/{} {}", f_name, n_name, k.name(), i);
                                tr.add(&format!("{}/{}/{} {}", f_name, n_name, k.name(), i));

                                for k2 in k.fields() {
                                    let v = k12.get_field(&k2);
                                    // draw(next_node, tr, dp, k, v)
                                    draw(next_node, MyTree { t: tr.t.clone() }, dp, k2, v.borrow());
                                    // for k99 in new_row {
                                    //     row_vec.push(k99);
                                    // }
                                }
                            }
                        }
                    }
                }

                // let b_new_row_vec = row_vec.clone();
                // let mut is_enable = false;

                // but.set_callback(move |_| {

                //     if !is_enable {
                //         is_enable = true;
                //         for mut l88 in b_new_row_vec.clone() {
                //             l88.deactivate();
                //             l88.hide();
                //         }
                //     } else {
                //         is_enable = false;
                //         for mut l88 in b_new_row_vec.clone() {
                //             l88.activate();
                //             l88.show();
                //         }
                //     }
                // });
            } else {
                // let _ = button::Button::new(160, 200, 80, 40, "empty");
                // row.end();
                // row.set_margins(pad, 0, 0, 0);
            }
        }
    }

    // let mut final_row_vec: Vec<group::Flex> = Vec::new();
    // // final_row_vec.push(row);

    // // for k89 in row_vec {
    // //     final_row_vec.push(k89);
    // // }

    // return final_row_vec;
}
