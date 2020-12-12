![crates.io](https://img.shields.io/crates/v/pydis.svg)

# pydis

A Rust crate for disassembling Python 2.7 bytecode

## Example Usage

```rust
use pydis::decode;
use pydis::opcode::*;

fn disassemble(bytecode: &[u8]) {
    let mut rdr = std::io::Cursor::new(bytecode);
    while let Ok(instr) = decode(&mut rdr) {
        println!("{:#?}", instr);
    }
}
```