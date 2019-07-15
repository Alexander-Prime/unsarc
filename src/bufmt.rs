pub struct Bufmt<'a>(pub &'a [u8]);

impl<'a> std::fmt::Display for Bufmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ ")
            .map(|_| {
                for &byte in self.0 {
                    write!(f, "{} ", byte);
                }
            })
            .and_then(|_| write!(f, "]"))
    }
}

impl<'a> std::fmt::LowerHex for Bufmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ ")
            .map(|_| {
                for &byte in self.0 {
                    write!(f, "{:02x} ", byte);
                }
            })
            .and_then(|_| write!(f, "]"))
    }
}
impl<'a> std::fmt::UpperHex for Bufmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ ")
            .map(|_| {
                for &byte in self.0 {
                    write!(f, "{:02X} ", byte);
                }
            })
            .and_then(|_| write!(f, "]"))
    }
}