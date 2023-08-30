use plain::Plain;
use scroll::{Pread, Pwrite, SizeWith};

/// The 64-bit mach header magic
pub const MH_MAGIC_64: u32 = 0xfeedfacf;
/// The 64-bit mach header magic LE
pub const MH_CIGAM_64: u32 = 0xcffaedfe;

/// The 32-bit mach header magic
/// Not supported but used for detection
pub const MH_MAGIC: u32 = 0xfeedface;
pub const MH_CIGAM: u32 = 0xcefaedfe;

pub const MH_64_SIZE: usize = 32;

#[repr(C)]
#[derive(Clone, Copy, Default, Pread, Pwrite, SizeWith)]
/// 64-bit Mach-o header
pub struct MachHeader64 {
    /// mach magic number identifier
    pub magic: u32,
    /// cpu specifier
    pub cputype: i32,
    /// machine specifier
    pub cpusubtype: i32,
    /// type of file
    pub filetype: u32,
    /// number of load commands
    pub ncmds: u32,
    /// the size of all the load commands
    pub sizeofcmds: u32,
    /// flags
    pub flags: u32,
    pub reserved: u32,
}

unsafe impl Plain for MachHeader64 {}

impl MachHeader64 {
    pub fn from_bytes(bytes: &[u8; MH_64_SIZE]) -> &Self {
        plain::from_bytes(bytes).unwrap()
    }
}

impl core::fmt::Debug for MachHeader64 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Header")
            .field("magic", &format_args!("0x{:x}", self.magic))
            .field("cputype", &format_args!("0x{:x}", self.cputype))
            .field("cpusubtype", &format_args!("0x{:x}", self.cpusubtype))
            .field("filetype", &format_args!("0x{:x}", self.filetype))
            .field("ncmds", &self.ncmds)
            .field("sizeofcmds", &self.sizeofcmds)
            .field("flags", &format_args!("0x{:x}", self.flags))
            .field("reserved", &format_args!("0x{:x}", self.reserved))
            .finish()
    }
}
