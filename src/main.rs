use anyhow::Result;
mod parser;
use parser::file::{check_file_extension, read_file_to_buffer};
use parser::lxob::{LXOB_HEADER_SIZE, LXO_FILE_EXTENSION};
use parser::{is_lxob, parse_chunk_headers, parse_chunk_pnts, parse_file_header};

// ===============
// Read: README.md
// ===============
// const EXPECTED_FILE_SIZE_IN_BYTES: usize = 18674;
// const EXPECTED_CHUNK_COUNT: usize = 49;
//
fn main() -> Result<()> {
    let filename = "cube.lxo";
    check_file_extension(filename, LXO_FILE_EXTENSION)?;

    let buffer = read_file_to_buffer(filename)?;
    println!("Bytes read: {:#?}", buffer.len());
    assert!(is_lxob(&buffer), "Is a LXOB file?");

    let (_, header) = parse_file_header(&buffer).expect("Parse the file's header.");
    println!("Header: {:#?}", header);

    let (_, chunks) =
        parse_chunk_headers(&buffer[LXOB_HEADER_SIZE..]).expect("Parse all the chunks.");
    println!("Chunk count: {:#?}", chunks.len());

    let (_, pnts_chunk) = parse_chunk_pnts(&buffer).expect("Extracted 3d point data.");
    println!("Points {:#?}", pnts_chunk);

    Ok(())
}
