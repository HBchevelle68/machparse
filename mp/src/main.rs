use machlib::headers::MachHeader64;

use scroll::Pread;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut bytes: [u8; 32] = [0; 32];

    let mut fd: File =
        File::open("/Users/ap/Documents/repos/machparse/machbin/cmach/cmach").unwrap();
    fd.read_exact(&mut bytes).unwrap();

    dbg!();

    let hdr = bytes.pread::<MachHeader64>(0).unwrap();

    dbg!(hdr);
}
