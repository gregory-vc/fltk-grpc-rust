use enums::Color;
use group::Flex;
use prost_reflect::DynamicMessage;
use prost_reflect::ReflectMessage;
use prost_reflect::FieldDescriptor;
use prost_reflect::Value;
use anyhow::Result;
use fltk::{prelude::*, *};

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


fn proto2dynamic(proto: impl ReflectMessage) -> Result<DynamicMessage> {
    Ok(DynamicMessage::decode(
        proto.descriptor(),
        proto.encode_to_vec().as_slice(),
    )?)
}

pub fn draw_proto(event: impl ReflectMessage) -> Result<()> {
    let message: prost_reflect::DynamicMessage = proto2dynamic(event)?;

    println!("start__--------------------------------------------------->");

    let mut col = group::Flex::default_fill().column();
    col.set_margin(20);

    for (k, v) in message.fields() {


        draw(&">".to_string(), k, v);


    }

    col.end();
    col.set_pad(30);

    Ok(())
}

fn draw(del: &String, k: FieldDescriptor, v: &Value) {
    // let mut row = group::Flex::default();
    // col.fixed(&row, 75);

    let next_del = del.to_owned()+">";
    if !k.is_list() {
        let name = del.to_owned() + k.full_name();
        let _ = MyFrame::new(&name, enums::Color::Light3);
    } else {
        let name = next_del.to_owned() + k.full_name();
        let _ = MyFrame::new(&name, enums::Color::Inactive);

        if let Some(v11) = v.as_list() {
            for k11 in v11.iter() {
                if let Some(k12) = k11.as_message() {
                    for (k13, v13) in k12.fields() {
                        // let mut col = group::Flex::default_fill().column();
                        // col.set_margin(20);

                        draw(&next_del, k13, v13);
                    }
                }else {
                    println!("------>> empty");
                }
            }
        } else {
            println!("------>> empty as_list");
        }
    }

    // row.end();
    // row.set_pad(10);

    // col.end();
    // col.set_pad(30);

}