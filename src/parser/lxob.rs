pub type ByteCount = usize;

pub const IFF_ID_FIELD_SIZE: ByteCount = 4;
pub const IFF_SIZE_FIELD_SIZE: ByteCount = 4;
pub const LXOB_HEADER_SIZE: ByteCount = 52;
pub const CHUNK_HEADER_SIZE: ByteCount = 8;

pub const BYTES_PER_LXOB_FLOAT: ByteCount = 4;
pub const BYTES_PER_3D_POINT: ByteCount = 3 * BYTES_PER_LXOB_FLOAT;

pub type Tag = &'static str;
pub const TAG_CHAR_LEN: usize = 4;
pub const LXOB_FILE_TYPE_TAG: Tag = "LXOB";
pub const PNTS_CHUNK_TAG: Tag = "PNTS"; // &[0x50, 0x4E, 0x54, 0x53]

pub const LXO_FILE_EXTENSION: &[&str] = &["lxo"];

/// Deserialised data from a .lxo (LXOB) file header.
//
// Other options for the 4-byte tags,
//  version_tag:  &'a[u8], // store shared-reference to byte slice
//  version_tag:  [u8],    // copy 4 bytes to array; e.g. header.version_tag.clone_from_slice(data);
//
#[derive(Debug)]
pub struct FileHeader {
    pub iff_id: String,
    pub byte_count: ByteCount,
    pub file_type_tag: String,
    pub version_tag: String,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub file_type_author: String,
}

impl FileHeader {
    fn new() -> Self {
        Self {
            iff_id: "".to_owned(),
            byte_count: 0,
            file_type_tag: "".to_owned(),
            version_tag: "".to_owned(),
            major: 0,
            minor: 0,
            patch: 0,
            file_type_author: "".to_owned(),
        }
    }
}

impl Default for FileHeader {
    fn default() -> Self {
        Self::new()
    }
}

/// Deserialised header from a single LXOB chunk.
//
#[derive(Debug)]
pub struct ChunkHeader {
    pub name: String,
    pub data_size: ByteCount,
}

impl ChunkHeader {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            data_size: 0,
        }
    }
}

impl Default for ChunkHeader {
    fn default() -> Self {
        Self::new()
    }
}

/// Deserialized [header](ChunkHeader) and data from a single LXOB chunk.
//
#[derive(Debug)]
pub struct Chunk<T> {
    pub header: ChunkHeader,
    pub data: Vec<T>,
}

#[allow(dead_code)]
impl<T> Chunk<T> {
    pub fn new() -> Self {
        Self {
            header: ChunkHeader::new(),
            data: Vec::<T>::new(),
        }
    }

    pub fn binary_size(self) -> ByteCount {
        CHUNK_HEADER_SIZE + self.header.data_size
    }
}

#[allow(dead_code)]
impl<T> Default for Chunk<T> {
    fn default() -> Self {
        Self::new()
    }
}
