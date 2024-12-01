pub struct Header {
    pub version: i32,
}

#[repr(C)]
pub struct CPadFile {
    pub header: Header,
}
