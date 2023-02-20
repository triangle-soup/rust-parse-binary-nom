use anyhow::{Error, Result};
mod reader;
use reader::{check_file_extension, read_file_to_buffer};
mod geom;
mod parser;
use parser::lxob::LXO_FILE_EXTENSION;
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
    if !is_lxob(&buffer) {
        return Err(Error::msg("LXOB file identifier not found."));
    }

    let (_, header) =
        parse_file_header(&buffer).map_err(|e| Error::msg(format!("Error parsing header: {e}")))?;
    println!("Header: {header:#?}");

    let (_, chunks) = parse_chunk_headers(&buffer)
        .map_err(|e| Error::msg(format!("Error parsing chunk headers: {e}")))?;
    println!("Chunk count: {:#?}", chunks.len());

    let (_, pnts_chunk) = parse_chunk_pnts(&buffer)
        .map_err(|e| Error::msg(format!("Error parsing PNTS chunk: {e}")))?;
    println!("Points: {pnts_chunk:#?}");

    Ok(())
}
