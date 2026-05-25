mod parser;

use parser::reader::Reader;

fn main() {
    println!("Hello, world!");

    let a = b"123";

    let mut reader = Reader::new(a);

    let json = reader.read();

    print!("{:?}", json)
}
