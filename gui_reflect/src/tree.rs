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

static COFACTOR: utils::oncelock::Lazy<i32> =
    utils::oncelock::Lazy::new(|| (app::font_size() as f64 * 3.0) as i32);

fn prep_tree(t: &mut tree::Tree) {
    if let Some(root) = t.next(&t.first().unwrap()) {
        if root.is_open() {
            let elems = root.children();
            t.resize(t.x(), t.y(), t.w(), (elems + 1) * *COFACTOR);
        } else {
            t.set_scrollbar_size(0);
            t.resize(t.x(), t.y(), t.w(), *COFACTOR);
        }
    } else {
        t.resize(t.x(), t.y(), t.w(), *COFACTOR);
    }
    app::redraw();
}

struct MyTree {
    t: tree::Tree,
}

impl MyTree {
    pub fn default() -> Self {
        let mut t = tree::Tree::default();
        t.set_show_root(false);
        t.set_callback(prep_tree);
        Self { t }
    }

    pub fn end(&mut self) {
        prep_tree(&mut self.t);
    }
}

widget_extends!(MyTree, tree::Tree, t);

fn proto2dynamic(proto: impl ReflectMessage) -> Result<DynamicMessage> {
    Ok(DynamicMessage::decode(
        proto.descriptor(),
        proto.encode_to_vec().as_slice(),
    )?)
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

    t.end();
    // col.end();
    // col.set_pad(10);

    std::result::Result::Ok(())
}

fn draw(f_name: &str, mut tr: MyTree, dp: &DescriptorPool, k: FieldDescriptor, v: &Value) {
    // let mut row: group::Flex = group::Flex::default().row();
    // let next_pad = pad + 40;
    // let mut row_vec: Vec<group::Flex> = Vec::new();

    if !k.is_list() {
        tr.add(&format!("{}/{}", f_name, k.full_name()));
        // let name = k.full_name();
        // let _ = MyFrame::new(&name, enums::Color::DarkMagenta);
        // let nn = format!("{:?}", k.kind());
        // let _ = MyFrame::new(&nn, enums::Color::DarkMagenta);
        // let _ = MyInput::new(v, nn, dp);

        // row.end();
        // row.set_margins(pad, 0, 0, 0);
    } else {
        // let name = k.full_name();
        // let _ = MyFrame::new(&name, enums::Color::Inactive);
        // let nn = format!("{:?}", k.kind());
        // let _ = MyFrame::new(&nn, enums::Color::Inactive);

        if let Some(v11) = v.as_list() {
            if v11.len() > 0 {
                // let mut but = button::Button::new(160, 200, 80, 40, ">");
                // row.end();
                // row.set_margins(pad, 0, 0, 0);
                for k11 in v11.iter() {
                    if let Some(k12) = k11.as_message() {
                        for k in dp.all_messages() {
                            if k12.descriptor().full_name() == k.full_name() {
                                for k2 in k.fields() {
                                    let v = k12.get_field(&k2);
                                    // let new_row = draw(next_pad, k2, v.borrow(), dp);
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
