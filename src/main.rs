mod bom;
mod sarcfile;

use sarcfile::SarcFile;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for path in &args[1..] {
        let file = SarcFile::open(path).unwrap();
        for node in file.nodes() {
            println!("{}", node);
        }
    }

}
