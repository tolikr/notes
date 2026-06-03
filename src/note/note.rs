use std::fmt;

pub struct Note {
    theme: String,
    text: String
}

impl Note {

    pub fn display_list(l: Vec<Note>) {
        for n in l {
            println!("{}\n", n)
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Пишем шаблон, как именно должна выглядеть строка
        write!(f, "📝 Theme: {}\nText: {}", self.theme, self.text)
    }
}