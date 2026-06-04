pub struct NoteManager;

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use crate::{note::note::Note, parser::reader::Reader};

impl NoteManager {
    const DATABASE: &str = "db.json";

    pub fn list() -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let db = Self::prepare_db()?;

        let notes: Vec<Note> = Self::read_nodes(&db)?;

        Ok(notes)
    }

    pub fn clean() -> Result<(), Box<dyn std::error::Error>> {
        let db = Self::prepare_db()?;

        std::fs::write(db, b"[]")?;

        Ok(())
    }

    pub fn add(theme: String, text: String) -> Result<(), Box<dyn std::error::Error>> {
        let db = Self::prepare_db()?;

        let mut notes: Vec<Note> = Self::read_nodes(&db)?;

        notes.push(Note {
            theme: theme,
            text: text,
        });

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(db)?;

        file.write_all(&Note::to_buffer(notes))?;

        Ok(())
    }

    pub fn remove(theme: &str) -> Result<(), Box<dyn std::error::Error>> {
        let db = Self::prepare_db()?;

        let notes: Vec<Note> = Self::read_nodes(&db)?;

        let notes: Vec<Note> = notes.into_iter().filter(|n| n.text != theme).collect();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(db)?;

        file.write_all(&Note::to_buffer(notes))?;
        
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

    fn read_nodes(db: &PathBuf) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let db = File::open(db)?;
        let mut buf_reader = BufReader::new(db);

        let mut buffer = Vec::new();
        buf_reader.read_to_end(&mut buffer)?; // Буфер создаётся и живет здесь

        let mut reader = Reader::new(&buffer);
        let json = reader.read().map_err(|e| {
            let msg = format!("Ошибка ввода-вывода ({:?}). JSON не прочитан!", e);
            Box::<dyn std::error::Error>::from(msg)
        })?;

        Note::from_json(json).map_err(|e| {
            let msg = format!("Notes reading error ({}).", e);
            Box::<dyn std::error::Error>::from(msg)
        })
    }
}
