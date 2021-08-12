![crates.io](https://img.shields.io/crates/v/pydis.svg)

# pydis

A Rust crate for disassembling Python 2.7 bytecode

## Example Usage

```rust
use pydis::prelude::*;
use pydis::opcode::py27::Standard;

fn disassemble(bytecode: &[u8]) {
    let mut rdr = std::io::Cursor::new(bytecode);
    // Decode using the standard Python 2.7 opcode table.
    // A custom opcode table can be passed by calling `decode::<OpcodeTable, _>(source)`
    while let Ok(instr) = decode_py27::<Standard, _>(&mut rdr) {
        println!("{:#?}", instr);
    }
}
```
