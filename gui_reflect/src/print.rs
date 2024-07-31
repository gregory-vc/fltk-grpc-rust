use anyhow::Result;
use prost_reflect::DynamicMessage;
use prost_reflect::FieldDescriptor;
use prost_reflect::ReflectMessage;
use prost_reflect::Value;

fn proto2dynamic(proto: impl ReflectMessage) -> Result<DynamicMessage> {
    Ok(DynamicMessage::decode(
        proto.descriptor(),
        proto.encode_to_vec().as_slice(),
    )?)
}

fn print_message(del: &String, k: FieldDescriptor, v: &Value) {
    let next_del = del.to_owned() + ">";
    if !k.is_list() {
        println!("{} {}, {}, {:?}", del, k.full_name(), v, k.kind());
    } else {
        println!("{} {}, {}, {:?}", next_del, k.full_name(), v, k.kind());

        if let Some(v11) = v.as_list() {
            for k11 in v11.iter() {
                if let Some(k12) = k11.as_message() {
                    for (k13, v13) in k12.fields() {
                        print_message(&next_del, k13, v13);
                    }
                } else {
                    println!("------>> empty");
                }
            }
        } else {
            println!("------>> empty as_list");
        }
    }
}

pub fn print_proto(event: impl ReflectMessage) -> Result<()> {
    let message: prost_reflect::DynamicMessage = proto2dynamic(event)?;

    println!("start--------------------------------------------------->");

    for (k, v) in message.fields() {
        print_message(&">".to_string(), k, v);
    }

    Ok(())
}
