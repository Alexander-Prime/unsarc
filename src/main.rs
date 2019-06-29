mod bom;
mod sarcfile;

use sarcfile::SarcFile;

fn main() {
    let file = SarcFile::open("./data.pack");
}
