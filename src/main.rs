mod bom;
mod sarcfile;

use sarcfile::SarcFile;

fn main() {
    let file = SarcFile::open("./data.pack").unwrap();

    for node in file.nodes() {
        println!("{}", node);
    }
}
