extern crate chrono;

use chrono::prelude::*;

use enums::Color;
use prost_reflect::DynamicMessage;
use prost_reflect::ReflectMessage;
use prost_reflect::FieldDescriptor;
use prost_reflect::Value;
use anyhow::Result;
use fltk::{prelude::*, *};
use fltk_calendar::calendar;

struct MyFrame {
    #[allow(dead_code)]
    f: frame::Frame,
}

impl MyFrame {
    pub fn new(f_name: &str, cl: Color) -> MyFrame {
        let mut f = frame::Frame::default();
        // Normally you would use the FrameType enum, for example:
        // some_widget.set_frame(FrameType::DownBox);
        f.set_frame(enums::FrameType::by_index(1));
        f.set_color(cl);
        f.set_label(f_name);
        f.set_label_size(12);
        Self { f }
    }
}

struct MyInput {
}

impl MyInput {
    pub fn new(v: &Value, k: String) -> MyInput {
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

            "google.protobuf.Timestamp" => {
                let mut ipt = input::Input::default();
                
                if let Some(k55) = v.as_message() {
                    if let Some(s55) = k55.get_field_by_name("seconds").as_ref() {
                        if let Some(vl) = s55.as_i64() {
                            if let Some(dt56) = DateTime::from_timestamp(vl, 0) {
                                ipt.set_value(dt56.format("%Y-%m-%d").to_string().as_str());
                            }
                        }
                    }
                }

                if let Some(vl) = v.as_f64() {
                    let s1 = vl.to_string();
                    ipt.set_value(s1.as_str());
                }
                
                let mut but = button::Button::new(160, 200, 80, 40, "change");
                but.set_callback(move |_| {
                    let cal = calendar::Calendar::default();
                    let date = cal.get_date();
                    println!("{:?}", date);
                    if let Some(date) = date {
                        println!("{:?}", date.to_string());
                        ipt.set_value(date.to_string().as_str());
                    }
                });
            }
            _ => println!("something else!"),
        }

        Self {  }
    }
}

fn proto2dynamic(proto: impl ReflectMessage) -> Result<DynamicMessage> {
    Ok(DynamicMessage::decode(
        proto.descriptor(),
        proto.encode_to_vec().as_slice(),
    )?)
}

pub fn draw_proto(event: impl ReflectMessage) -> Result<()> {
    let message: prost_reflect::DynamicMessage = proto2dynamic(event)?;
    let mut col = group::Flex::default_fill().column();
    col.set_margin(10);

    for (k, v) in message.fields() {
        draw(&">".to_string(), k, v);
    }

    col.end();
    col.set_pad(10);

    Ok(())
}

fn draw(del: &String, k: FieldDescriptor, v: &Value) {
    let mut row = group::Flex::default();

    let next_del = del.to_owned()+">";
    if !k.is_list() {
        let name = del.to_owned() + k.full_name();
        let _ = MyFrame::new(&name, enums::Color::Light3);

        let nn = format!("{:?}", k.kind());
        let _ = MyFrame::new(&nn, enums::Color::Light3);

        let _ = MyInput::new(v, nn);

        row.end();
        row.set_pad(10);
    } else {
        let name = next_del.to_owned() + k.full_name();
        let _ = MyFrame::new(&name, enums::Color::Inactive);

        let nn = format!("{:?}", k.kind());
        let _ = MyFrame::new(&nn, enums::Color::Inactive);

        row.end();
        row.set_pad(10);

        if let Some(v11) = v.as_list() {
            for k11 in v11.iter() {
                if let Some(k12) = k11.as_message() {
                    for (k13, v13) in k12.fields() {
                        draw(&next_del, k13, v13);
                    }
                }
            }
        }
    }
}