pub struct NoteManager;

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write}, path::Path,
};

use crate::{note::note::Note, parser::reader::Reader};

impl NoteManager {
    const DATABASE: &str = "db.json";

    pub fn list() -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        if !Path::new(Self::DATABASE).exists() {
             // Вместо File::open используем OpenOptions:
            let mut file = OpenOptions::new()
                .read(true) // Нам нужно читать из него данные
                .write(true) // (Опционально) если потом захочешь записывать
                .create(true) // Создать файл, если его не существует!
                .open(Self::DATABASE)?;

            file.write("[]".as_bytes())?;
        }

        let db = File::open(Self::DATABASE)?;

        let mut buf_reader = BufReader::new(db);

        let mut buffer = Vec::new();
        buf_reader.read_to_end(&mut buffer)?; // Буфер создаётся и живет здесь

        let mut reader = Reader::new(&buffer);
        let json = reader.read().map_err(|e| {
            let msg = format!("Ошибка ввода-вывода ({:?}). JSON не прочитан!", e);
            Box::<dyn std::error::Error>::from(msg)
        })?;

        let notes: Vec<Note> = Note::from_json(json).map_err(|e| {
            let msg = format!("Notes reading error ({}).", e);
            Box::<dyn std::error::Error>::from(msg)
        })?;

        Ok(notes)
    }
}
