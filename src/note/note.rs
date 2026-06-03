use std::fmt;

use crate::parser::json::Json;

pub struct Note {
    theme: String,
    text: String,
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
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Пишем шаблон, как именно должна выглядеть строка
        write!(f, "📝 Theme: {}\nText: {}", self.theme, self.text)
    }
}
