mod py27;

pub use py27::*;

#[derive(Debug, Clone)]
pub struct Instruction<O: Opcode> {
    pub opcode: O,
    pub arg: Option<u16>,
}

/// Trait that provides convenience routines for opcode properties such as whether
/// or not it has an argument, is a jump, etc.
pub trait Opcode {
    /// Whether or not this opcode has an argument
    fn has_arg(&self) -> bool;

    /// Whether or not this opcode has an extended argument
    fn has_extended_arg(&self) -> bool;

    /// Whether or not this opcode has a constant parameter
    fn has_const(&self) -> bool;

    /// Whether or not this opcode is a boolean operation
    fn has_comp(&self) -> bool;

    /// Whether or not this opcode has a relative jump target
    fn is_relative_jump(&self) -> bool;

    /// Whether or not this opcode has an absolute jump target
    fn is_absolute_jump(&self) -> bool;

    /// Whether or not this opcode is a conditional jump
    fn is_conditional_jump(&self) -> bool;

    /// Whether or not this opcode accesses an attribute by name
    fn has_name(&self) -> bool;

    /// Whether or not this opcode accesses a local variable
    fn has_local(&self) -> bool;

    /// Whether or not this opcode accesses a free variable
    fn has_free(&self) -> bool;
}
