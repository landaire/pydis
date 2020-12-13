![crates.io](https://img.shields.io/crates/v/pydis.svg)

# pydis

A Rust crate for disassembling Python 2.7 bytecode

## Example Usage

```rust
use pydis::prelude::*;

fn disassemble(bytecode: &[u8]) {
    let mut rdr = std::io::Cursor::new(bytecode);
    // Optionally an explicit/custom opcode table can be passed by calling `decode::<OpcodeTable, _>(source)`
    while let Ok(instr) = decode_py27(&mut rdr) {
        println!("{:#?}", instr);
    }
}
```
