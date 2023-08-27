use scroll::{Pread, Pwrite, SizeWith};

/// The 64-bit mach header magic
pub const MH_MAGIC_64: u32 = 0xfeedfacf;
/// The 64-bit mach header magic LE
pub const MH_CIGAM_64: u32 = 0xcffaedfe;

/// The 32-bit mach header magic
/// Not supported but used for detection
pub const MH_MAGIC: u32 = 0xfeedface;
pub const MH_CIGAM: u32 = 0xcefaedfe;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Pread, Pwrite, SizeWith)]
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

impl MachHeader64 {}
