pub struct NoteManager;

use std::{fs::File, io::{BufReader, Read}};

use crate::note::note::Note;

impl NoteManager {
    const DATABASE: &str = "db.json";

    pub fn list() -> Result<Vec<Note>, Box<dyn std::error::Error>> {

        let db = File::open(Self::DATABASE)?;

        let reader = BufReader::new(db);

        let notes: Vec<Note> = ?;

        Ok(notes)
    }
}