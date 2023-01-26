[![Rust](https://github.com/triangle-soup/rust-parse-binary-nom/actions/workflows/rust.yml/badge.svg)](https://github.com/triangle-soup/rust-parse-binary-nom/actions/workflows/rust.yml)

A fun exercise that I created to learn a little bit about using *[nom](https://crates.io/crates/nom), a parser combinators library for Rust*.

### **Aims** 
Parse the cube.lxo binary file (LXOB; IFF file format) and,
 1. Deserialize the file header. 
 2. Deserialize all chunk headers; i.e. the name-tag and size, in bytes, of each chunk.
 3. Deserialize the 3d point data from the PNTS chunk.

The scope of this exercise did not include the creation of a comprehensive LXOB file parser.

### **LXOB Chunk Format**

    tag[ID4]: 4-byte; Four upper-case ASCII characters; e.g. PNTS.
    size[U4]: 4-byte; unsigned int; sub-chunks size[U2].
    data:     data[size]; i.e. size bytes of data.
    pad:      padding byte added if size is odd, for even byte boundary.

Note, "All binary datatypes are stored in Motorola byte order, also known as big endian or network order, with the most significant byte first.", modo SDK docs [[1]](#iff-file-format-refs).

### **File Header Example**

Here's the cube.lxo file's header, found by parsing the first 52 bytes,

    FORM  - IFF identifier (ID). This must be either FORM or LIST or 'CAT '
    18666 - byte count after this point to EOF; i.e. excl. first 8 bytes; File is 18674 bytes.
    LXOB
    VRSN
    32
    4
    1
    nexu
    s 10
     by
    The
    Foun
    dry\0

### **Tips**
 - Show the file's byte count on Linux: wc -c < cube.lxo

 - Use software like the "010 Editor"--there's a template for modo files--to
   inspect the cube.lxo binary file.

### **IFF File Format**
- [IFF format](https://en.wikipedia.org/wiki/Interchange_File_Format)
- [LXOB docs (incomplete)](https://learn.foundry.com/modo/developers/latest/sdk/pages/general/general/File%20Formats.html)

### **nom**
- [nom crate](https://crates.io/crates/nom)
- [choosing a combinator](https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md)
- [nom recipies](https://docs.rs/nom/7.1.2/nom/recipes/index.html)
