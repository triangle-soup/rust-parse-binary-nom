pub mod lxob;
use lxob::*;
use crate::geom::Point;

use nom::{
    bytes::complete::{take, take_until},
    multi::{count, many1},
    number::complete::{be_f32, be_u32},
    IResult,
};

use std::convert::TryFrom;
use std::str;

/// Returns the deserialised header of a .lxo (LXOB) file, which consists of 52 bytes. See, README.md
//
pub fn parse_file_header(input: &[u8]) -> IResult<&[u8], FileHeader> {
    let (remains, iff_id) = parse_tag(input)?;
    let (remains, byte_count) = parse_usize(remains)?;
    let (remains, file_type_tag) = parse_tag(remains)?;
    let (remains, version_tag) = parse_tag(remains)?;
    let (remains, major) = parse_u32(remains)?;
    let (remains, minor) = parse_u32(remains)?;
    let (remains, patch) = parse_u32(remains)?;
    let (remains, file_type_author) = parse_file_type_author(remains)?;
    let (remains, _) = take(1usize)(remains)?; // skip null byte at end of header.

    Ok((
        remains,
        FileHeader {
            iff_id,
            byte_count,
            file_type_tag,
            version_tag,
            major,
            minor,
            patch,
            file_type_author,
        },
    ))
}

/// IFF integers are stored big-endian, hence, must use 'be_u32' (avoid 'le_u..') from nom library.
// ...see https://uynguyen.github.io/2018/04/30/Big-Endian-vs-Little-Endian/
//
fn parse_u32(input: &[u8]) -> IResult<&[u8], u32> {
    be_u32(input)
}

fn parse_tag(input: &[u8]) -> IResult<&[u8], &str> {
    let (remains, tag) = take(4usize)(input)?;
    Ok((remains, to_str(tag)))
}

fn parse_usize(input: &[u8]) -> IResult<&[u8], usize> {
    let (remains, parsed_u32) = parse_u32(input)?;
    Ok((remains, to_usize(parsed_u32)))
}

fn to_str(val: &[u8]) -> &str {
    match str::from_utf8(val) {
        Ok(as_string) => as_string,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

fn to_usize(val: u32) -> usize {
    match usize::try_from(val) {
        Ok(as_usize) => as_usize,
        Err(e) => panic!("usize unable to contain u32 {}", e),
    }
}

fn parse_file_type_author(input: &[u8]) -> IResult<&[u8], &str> {
    let (remains, author) = take_until("\0")(input)?;
    Ok((remains, to_str(author)))
}

fn parse_chunk_header(input: &[u8]) -> IResult<&[u8], ChunkHeader> {
    let (remains, name) = parse_tag(input)?;
    let (remains, data_size) = parse_usize(remains)?;
    Ok((remains, ChunkHeader { name, data_size }))
}

fn skip_chunk_data(input: &[u8], data_size: ByteCount) -> IResult<&[u8], &[u8]> {
    // skip to next chunk; add 1 (for 0 pad byte) if size_in_bytes is odd as
    // chunks must end on even boundary. this should be case when the file was saved.
    //
    let mut skip_bytes_count = data_size;
    if data_size % 2 != 0 {
        skip_bytes_count += 1;
    }
    let (remains, data) = take(skip_bytes_count)(input)?;
    Ok((remains, data))
}

fn parse_chunk_header_skip_data(input: &[u8]) -> IResult<&[u8], ChunkHeader> {
    let (remains, header) = parse_chunk_header(input)?;
    let (remains, _) = skip_chunk_data(remains, header.data_size)?;
    Ok((remains, header))
}

/// Parses only the header of each chunk; i.e. ignores/skips the data.
//
pub fn parse_chunk_headers(input: &[u8]) -> IResult<&[u8], Vec<ChunkHeader>> {
    let (input, chunks) = many1(parse_chunk_header_skip_data)(input)?;
    Ok((input, chunks))
}

fn take_until_chunk(input: &[u8], tag: Tag) -> IResult<&[u8], &[u8]> {
    assert!(tag.len() == TAG_CHAR_LEN);
    let (remains, taken) = take_until(tag.as_bytes())(input)?;
    Ok((remains, taken))
}

fn parse_point(input: &[u8]) -> IResult<&[u8], Point> {
    let (input, x) = be_f32(input)?;
    let (input, y) = be_f32(input)?;
    let (input, z) = be_f32(input)?;
    Ok((input, Point { x, y, z }))
}

fn parse_points(input: &[u8], data_size: ByteCount) -> IResult<&[u8], Vec<Point>> {
    assert!(data_size >= BYTES_PER_3D_POINT);
    assert!(data_size % BYTES_PER_3D_POINT == 0);

    let point_count = data_size / BYTES_PER_3D_POINT;
    let (input, points) = count(parse_point, point_count)(input)?;
    Ok((input, points))
}

/// Parse the PNTS chunk. A 3d point consists of three f32 floats.
pub fn parse_chunk_pnts(input: &[u8]) -> IResult<&[u8], Chunk<Point>> {
    let (remains, _) = take_until_chunk(input, PNTS_CHUNK_TAG)?;

    let (remains, header) = parse_chunk_header(remains)?;
    assert_eq!(PNTS_CHUNK_TAG, header.name);

    let (remains, data) = parse_points(remains, header.data_size)?;

    Ok((remains, Chunk::<Point> { header, data }))
}

/// Returns true if the data contains the LXOB file type speficier.
pub fn is_lxob(input: &[u8]) -> bool {
    let (_, tag) = parse_tag(&input[IFF_ID_FIELD_SIZE + IFF_SIZE_FIELD_SIZE..])
        .expect("A 4-character alphabetic tag");
    tag == LXOB_FILE_TYPE_TAG
}

// a few tests; far from comprehensive.
//
#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const TEST_INPUT_HEADER: [u8; 52] = [
        70,  79,  82,  77,
        0,   0,   72,  234,
        76,  88,  79,  66,
        86,  82,  83,  78,
        0,   0,   0,   32,
        0,   0,   0,   4,
        0,   0,   0,   1,
        110, 101, 120, 117,
        115, 32,  49,  48,
        32,  98,  121, 32,
        84,  104, 101, 32,
        70,  111, 117, 110,
        100, 114, 121, 0
    ];

    const TEST_INPUT_TAG: &str = "ARTS";
    const TEST_INPUT_BYTE_COUNT: u32 = 15432;
    const TEST_INPUT_FILE_TYPE_AUTHOR: &str = "Who created this?\0";

    #[test]
    fn file_header_empty() {
        let result = parse_file_header(b"");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn file_header_incomplete() {
        let result = parse_file_header(b"FORM0000LXOBVRSN003200040001");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    #[should_panic]
    fn file_header_fields_out_of_order() {
        let mut header = TEST_INPUT_HEADER;
        // swap byte_count field with file_type_tag(i.e. LXOB)
        for i in 4..8 {
            header.swap(i, i + 4);
        }

        let result = parse_file_header(&header);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn file_header_good_input() {
        let result = parse_file_header(&TEST_INPUT_HEADER);
        assert_eq!(result.is_err(), false);

        let (_, header) = result.unwrap();
        assert_eq!("FORM", header.iff_id);
        assert_eq!(18666, header.byte_count);
        assert_eq!("LXOB", header.file_type_tag);
        assert_eq!("VRSN", header.version_tag);
        assert_eq!(32, header.major);
        assert_eq!(4, header.minor);
        assert_eq!(1, header.patch);
        assert_eq!("nexus 10 by The Foundry", header.file_type_author);
    }

    #[test]
    fn file_type_author_good_input() {
        let result = parse_file_type_author(TEST_INPUT_FILE_TYPE_AUTHOR.as_bytes());
        assert_eq!(result.is_err(), false);

        let (_, author) = result.unwrap();
        assert_eq!(TEST_INPUT_FILE_TYPE_AUTHOR.trim_end_matches("\0"), author);
    }

    #[test]
    #[should_panic]
    fn tag_not_alphabetic() {
        let input: &[u8; 4] = &(TEST_INPUT_BYTE_COUNT).to_be_bytes();
        assert_eq!(parse_tag(input).is_err(), true);
    }

    #[test]
    fn tag_is_alphabetic() {
        let result = parse_tag(TEST_INPUT_TAG.as_bytes());
        assert!(result.is_err() == false);
        let (_, tag) = result.unwrap();
        assert_eq!(TEST_INPUT_TAG, tag);
    }
}
