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

struct MyFrame {
    #[allow(dead_code)]
    f: frame::Frame,
}

impl MyFrame {
    pub fn new(f_name: &str, cl: Color) -> MyFrame {
        let mut f = frame::Frame::default();
        f.set_frame(enums::FrameType::by_index(1));
        f.set_color(cl);
        f.set_label(f_name);
        f.set_label_size(12);
        f.set_align(Align::Center);

        Self { f }
    }
}

struct MyInput {}

impl MyInput {
    pub fn new(v: &Value, k: String, dp: &DescriptorPool) -> MyInput {
        match k.as_str() {
            "string" => {
                let mut ipt = input::Input::default();
                if let Some(vl) = v.as_str() {
                    ipt.set_value(vl);
                }
            }

            "uint64" => {
                let mut ipt = input::FloatInput::default();
                if let Some(vl) = v.as_u64() {
                    ipt.set_value(vl.to_string().as_str());
                }
            }

            "float" => {
                let mut ipt = input::FloatInput::default();
                if let Some(vl) = v.as_f32() {
                    ipt.set_value(vl.to_string().as_str());
                }
            }

            "bool" => {
                let mut but3 = button::CheckButton::default().with_label("enabled");
                if let Some(vl) = v.as_bool() {
                    but3.set_value(vl);
                }
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
            }
            _ => {
                if let Some(vv1) = v.as_enum_number() {
                    if let Some(en) = dp.get_enum_by_name(k.as_str()) {
                        let mut chce = Choice::default();
                        for v55 in en.values() {
                            chce.add_choice(v55.name());
                        }
                        chce.set_value(vv1);
                    }
                }
            }
        }

        Self {}
    }
}

fn proto2dynamic(proto: impl ReflectMessage) -> Result<DynamicMessage> {
    Ok(DynamicMessage::decode(
        proto.descriptor(),
        proto.encode_to_vec().as_slice(),
    )?)
}

pub fn draw_proto(event: impl ReflectMessage, dp: &DescriptorPool) -> Result<()> {
    let message: prost_reflect::DynamicMessage = proto2dynamic(event)?;
    let mut col = group::Flex::default_fill().column();
    col.set_margin(10);

    for k in dp.all_messages() {
        if message.descriptor().full_name() == k.full_name() {
            for k2 in k.fields() {
                let v = message.get_field(&k2);
                draw(10, k2, v.borrow(), dp);
            }
        }
    }

    col.end();
    col.set_pad(10);

    std::result::Result::Ok(())
}

fn draw(pad: i32, k: FieldDescriptor, v: &Value, dp: &DescriptorPool) -> Vec<group::Flex> {
    let mut row: group::Flex = group::Flex::default().row();
    let next_pad = pad + 40;
    let mut row_vec: Vec<group::Flex> = Vec::new();

    if !k.is_list() {
        let name = k.full_name();
        let _ = MyFrame::new(&name, enums::Color::DarkMagenta);
        let nn = format!("{:?}", k.kind());
        let _ = MyFrame::new(&nn, enums::Color::DarkMagenta);
        let _ = MyInput::new(v, nn, dp);

        row.end();
        row.set_margins(pad, 0, 0, 0);
    } else {
        let name = k.full_name();
        let _ = MyFrame::new(&name, enums::Color::Inactive);
        let nn = format!("{:?}", k.kind());
        let _ = MyFrame::new(&nn, enums::Color::Inactive);

        if let Some(v11) = v.as_list() {
            if v11.len() > 0 {
                let mut but = button::Button::new(160, 200, 80, 40, ">");
                row.end();
                row.set_margins(pad, 0, 0, 0);
                for k11 in v11.iter() {
                    if let Some(k12) = k11.as_message() {
                        for k in dp.all_messages() {
                            if k12.descriptor().full_name() == k.full_name() {
                                for k2 in k.fields() {
                                    let v = k12.get_field(&k2);
                                    let new_row = draw(next_pad, k2, v.borrow(), dp);
                                    for k99 in new_row {
                                        row_vec.push(k99);
                                    }
                                }
                            }
                        }
                    }
                }

                let b_new_row_vec = row_vec.clone();
                let mut is_enable = false;

                but.set_callback(move |_| {
                    if !is_enable {
                        is_enable = true;
                        for mut l88 in b_new_row_vec.clone() {
                            l88.deactivate();
                            l88.hide();
                        }
                    } else {
                        is_enable = false;
                        for mut l88 in b_new_row_vec.clone() {
                            l88.activate();
                            l88.show();
                        }
                    }
                });
            } else {
                let _ = button::Button::new(160, 200, 80, 40, "empty");
                row.end();
                row.set_margins(pad, 0, 0, 0);
            }
        }
    }

    let mut final_row_vec: Vec<group::Flex> = Vec::new();
    final_row_vec.push(row);

    for k89 in row_vec {
        final_row_vec.push(k89);
    }

    return final_row_vec;
}
