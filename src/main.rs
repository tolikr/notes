mod parser;
mod input;

use parser::reader::Reader;
use input::console::Console;

fn main() {
    Console::run();

    let a = b"123";

    let mut reader = Reader::new(a);

    let json = reader.read();

    print!("{:?}", json)
}
