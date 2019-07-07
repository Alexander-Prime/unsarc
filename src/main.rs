mod bom;
mod sarc;

use sarc::reader::SarcReader;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for path in &args[1..] {
        let file = SarcReader::open(path).unwrap();
        for node in file.nodes() {
            println!("{}", node);
        }
    }
}
