use std::collections::HashMap;
use anyhow::Result;
use prost_reflect::{DynamicMessage, ReflectMessage};


pub fn proto2dynamic(proto: impl ReflectMessage) -> Result<DynamicMessage> {
    Ok(DynamicMessage::decode(
        proto.descriptor(),
        proto.encode_to_vec().as_slice(),
    )?)
}

pub fn value2kv(value: serde_json::Value) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    fn crawl(prefix: String, map: &mut HashMap<String, String>, value: serde_json::Value) {
        fn dot(prefix: &str, next: String) -> String {
            if prefix.is_empty() {
                next.to_string()
            } else {
                format!("{}.{}", prefix, next)
            }
        }

        match value {
            serde_json::Value::Null => {
                map.insert(prefix, "null".to_string());
            }

            serde_json::Value::String(value) => {
                map.insert(prefix, value);
            }

            serde_json::Value::Bool(value) => {
                map.insert(prefix, value.to_string());
            }

            serde_json::Value::Number(value) => {
                map.insert(prefix, value.to_string());
            }

            serde_json::Value::Array(value) => {
                for (i, item) in value.into_iter().enumerate() {
                    let next = dot(&prefix, i.to_string());
                    crawl(next, map, item);
                }
            }

            serde_json::Value::Object(value) => {
                for (key, item) in value {
                    let next = dot(&prefix, key);
                    crawl(next, map, item);
                }
            }
        }
    }
    crawl("".to_string(), &mut map, value);
    Ok(map)
}

// pub fn proto2kv(proto: impl ReflectMessage) -> Result<HashMap<String, String>> {
//     let message = proto2dynamic(proto)?;
//     let value = serde_json::to_value(message)?;
//     value2kv(value)
// }

pub fn kv2line(map: HashMap<String, String>) -> String {
    map.iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join(" ")
}

