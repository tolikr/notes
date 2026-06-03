pub struct NoteManager;

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write}, path::{Path, PathBuf},
};

use crate::{note::note::Note, parser::reader::Reader};

impl NoteManager {
    const DATABASE: &str = "db.json";

    pub fn list() -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let db = Self::prepare_db()?;
        let db = File::open(db)?;

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

    pub fn clean() -> Result<(), Box<dyn std::error::Error>> {
        let db = Self::prepare_db()?;

        std::fs::write(db, b"[]")?;

        Ok(())
    }

    pub fn add(theme: String, text: String) -> Result<(), std::io::Error> {


        Ok(())
    }

    pub fn remove(theme: &str) -> Result<(), std::io::Error> {


        Ok(())
    }

    fn prepare_db() -> Result<PathBuf, std::io::Error> {
        // Проверяем, существует ли путь
        if !Path::new(Self::DATABASE).exists() {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(Self::DATABASE)?;

            // Используем write_all вместо write
            file.write_all(b"[]")?; 
        }

        // Превращаем константу в PathBuf и оборачиваем в Ok
        Ok(PathBuf::from(Self::DATABASE))
    }
}
