use std::{collections::BTreeMap, fmt};

use crate::parser::{json::Json, writer::Writer};

pub struct Note {
    pub theme: String,
    pub text: String,
}

impl Note {
    pub fn display_list(l: Vec<Note>) {
        for n in l {
            println!("{}\n", n)
        }
    }

    pub fn from_json(j: Json) -> Result<Vec<Note>, String> {
        let el = match j {
            Json::Array(el) => el,
            _ => return Err("Not an array".to_string()),
        };

        el.into_iter()
            .map(|e| match e {
                Json::Object(obj) => {
                    let theme = obj.get("theme").ok_or("Missing theme")?;
                    let text = obj.get("text").ok_or("Missing text")?;

                    Ok(Note {
                        theme: format!("{:?}", theme),
                        text: format!("{:?}", text),
                    })
                }
                _ => Err("Not an object".to_string()),
            })
            .collect() // Магия: если хоть один элемент вернет Err, весь collect вернет Err
    }

    pub fn to_buffer(notes: Vec<Note>) -> Vec<u8> {
        let mut arr: Vec<Json> = Vec::new();

        for note in notes {
            let mut obj = BTreeMap::new();

            obj.insert("theme".to_string(), Json::String(note.theme));
            obj.insert("text".to_string(), Json::String(note.text));

            arr.push(Json::Object(obj));
        }

        Writer::write(Json::Array(arr))
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Пишем шаблон, как именно должна выглядеть строка
        write!(f, "📝 Theme: {}\nText: {}", self.theme, self.text)
    }
}
