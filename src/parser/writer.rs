use crate::parser::json::Json;

pub struct Writer {}

impl Writer {
    pub fn write(json: Json) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        fn write_string(str: String, buffer: &mut Vec<u8>) {
            buffer.push(b'"');
            buffer.extend_from_slice(str.as_bytes());
            buffer.push(b'"');
        }

        fn go(json: Json, buffer: &mut Vec<u8>) {
            match json {
                Json::Null => buffer.extend_from_slice(b"null"),
                Json::String(str) => write_string(str, buffer),
                Json::Bool(b) => {
                    if b {
                        buffer.extend_from_slice(b"true")
                    } else {
                        buffer.extend_from_slice(b"false")
                    }
                }
                Json::Number(n) => buffer.extend_from_slice(n.to_string().as_bytes()),
                Json::Array(arr) => {
                    buffer.extend_from_slice(b"[");

                    let mut first = true;
                    for js in arr {
                        if !first {
                            buffer.push(b',');
                        }
                        first = false;

                        go(js, buffer);
                    }

                    buffer.extend_from_slice(b"]");
                }
                Json::Object(obj) => {
                    buffer.extend_from_slice(b"{");

                    let mut first = true;
                    for (key, js) in obj {
                        if !first {
                            buffer.push(b',');
                        }
                        first = false;

                        write_string(key, buffer);

                        buffer.extend_from_slice(b":");

                        go(js, buffer);
                    }

                    buffer.extend_from_slice(b"}");
                }
            }
        }

        go(json, &mut buffer);

        buffer
    }
}
