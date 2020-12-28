use std::fmt::{self, Debug};

mod py27;

pub use py27::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction<O: Opcode> {
    pub opcode: O,
    pub arg: Option<u16>,
}

impl<O: Opcode + Debug> fmt::Display for Instruction<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.opcode)?;

        if let Some(arg) = self.arg {
            write!(f, " {}", arg)?;
        }

        Ok(())
    }
}

impl<O: Opcode> Instruction<O> {
    /// The length of this instruction in bytes
    pub fn len(&self) -> usize {
        std::mem::size_of::<u8>()
            + if let Some(arg) = self.arg.as_ref() {
                std::mem::size_of_val(arg)
            } else {
                0
            }
    }
}

impl Instruction<Python27> {
    /// How the stack adjusts after this instruction executes. A positive number indicates that N
    /// elements were pushed to the stack while a negative number indicates the number of elements
    /// removed from the stack
    pub fn stack_adjustment_after(&self) -> isize {
        type TargetOpcode = Python27;
        match self.opcode {
            // Meta instructions
            TargetOpcode::STOP_CODE
            | TargetOpcode::NOP
            | TargetOpcode::ROT_TWO
            | TargetOpcode::ROT_THREE
            | TargetOpcode::ROT_FOUR => 0,

            TargetOpcode::POP_TOP => -1,
            TargetOpcode::DUP_TOP => 1,
            TargetOpcode::DUP_TOP_TWO => 2,
            // Unary ops
            TargetOpcode::UNARY_POSITIVE
            | TargetOpcode::UNARY_NEGATIVE
            | TargetOpcode::UNARY_NOT
            | TargetOpcode::UNARY_CONVERT
            | TargetOpcode::UNARY_INVERT
            | TargetOpcode::GET_ITER => 0,
            // Binary ops
            TargetOpcode::BINARY_POWER
            | TargetOpcode::BINARY_MULTIPLY
            | TargetOpcode::BINARY_DIVIDE
            | TargetOpcode::BINARY_FLOOR_DIVIDE
            | TargetOpcode::BINARY_TRUE_DIVIDE
            | TargetOpcode::BINARY_MODULO
            | TargetOpcode::BINARY_ADD
            | TargetOpcode::BINARY_SUBTRACT
            | TargetOpcode::BINARY_SUBSC
            | TargetOpcode::BINARY_LSHIFT
            | TargetOpcode::BINARY_RSHIFT
            | TargetOpcode::BINARY_AND
            | TargetOpcode::BINARY_XOR
            | TargetOpcode::BINARY_OR => -1,
            // In-place operations
            TargetOpcode::INPLACE_POWER
            | TargetOpcode::INPLACE_MULTIPLY
            | TargetOpcode::INPLACE_DIVIDE
            | TargetOpcode::INPLACE_FLOOR_DIVIDE
            | TargetOpcode::INPLACE_TRUE_DIVIDE
            | TargetOpcode::INPLACE_MODULO
            | TargetOpcode::INPLACE_ADD
            | TargetOpcode::INPLACE_SUBTRACT
            | TargetOpcode::INPLACE_LSHIFT
            | TargetOpcode::INPLACE_RSHIFT
            | TargetOpcode::INPLACE_AND
            | TargetOpcode::INPLACE_XOR
            | TargetOpcode::INPLACE_OR => -1,
            // Slice operations
            TargetOpcode::SLICE_0 => 0,
            TargetOpcode::SLICE_1 => -1,
            TargetOpcode::SLICE_2 => -2,
            TargetOpcode::SLICE_3 => -3,
            TargetOpcode::STORE_SLICE_0 => -1,
            TargetOpcode::STORE_SLICE_1 => -2,
            TargetOpcode::STORE_SLICE_2 => -3,
            TargetOpcode::STORE_SLICE_3 => -4,
            TargetOpcode::DELETE_SLICE_0 => -1,
            TargetOpcode::DELETE_SLICE_1 => -2,
            TargetOpcode::DELETE_SLICE_2 => -3,
            TargetOpcode::DELETE_SLICE_3 => -4,
            TargetOpcode::STORE_SUBSCR => -3,
            TargetOpcode::DELETE_SUBSCR => -2,
            // Misc
            TargetOpcode::PRINT_EXPR => -1,
            TargetOpcode::PRINT_ITEM => -1,
            TargetOpcode::PRINT_ITEM_TO => -2,
            TargetOpcode::PRINT_NEWLINE => 0,
            TargetOpcode::PRINT_NEWLINE_TO => -1,
            TargetOpcode::BREAK_LOOP => 0,
            TargetOpcode::CONTINUE_LOOP => 0,
            TargetOpcode::LIST_APPEND => -1,
            TargetOpcode::LOAD_LOCALS => 1,
            TargetOpcode::RETURN_VALUE => 0,
            TargetOpcode::YIELD_VALUE => 0,
            TargetOpcode::IMPORT_STAR => -1,
            TargetOpcode::EXEC_STMT => -3,
            TargetOpcode::POP_BLOCK => 0,
            TargetOpcode::END_FINALLY => 0,
            TargetOpcode::BUILD_CLASS => -3,
            // TODO: maybe not right?
            TargetOpcode::SETUP_WITH => 1,
            TargetOpcode::WITH_CLEANUP => {
                panic!("with_cleanup may require runtime info");
            }
            TargetOpcode::STORE_NAME => -1,
            TargetOpcode::STORE_FAST => -1,
            TargetOpcode::STORE_DEREF => -1,
            TargetOpcode::SET_ADD => -1,
            TargetOpcode::MAP_ADD => -1,
            TargetOpcode::DELETE_NAME => 0,
            TargetOpcode::UNPACK_SEQUENCE => (self.arg.unwrap() as isize) - 1,
            TargetOpcode::DUP_TOPX => self.arg.unwrap() as isize,
            TargetOpcode::STORE_ATTR => -2,
            TargetOpcode::DELETE_ATTR => -1,
            TargetOpcode::STORE_GLOBAL => -1,
            TargetOpcode::DELETE_GLOBAL => 0,
            TargetOpcode::LOAD_CONST => 1,
            TargetOpcode::LOAD_NAME => 1,
            TargetOpcode::BUILD_TUPLE | TargetOpcode::BUILD_LIST | TargetOpcode::BUILD_SET => {
                (self.arg.unwrap() as isize) - 1
            }
            TargetOpcode::BUILD_MAP => 1,
            TargetOpcode::LOAD_ATTR => 0,
            TargetOpcode::COMPARE_OP => -1,
            TargetOpcode::IMPORT_NAME => -1,
            TargetOpcode::IMPORT_FROM => 1,
            TargetOpcode::JUMP_FORWARD | TargetOpcode::JUMP_ABSOLUTE => 0,
            TargetOpcode::POP_JUMP_IF_FALSE | TargetOpcode::POP_JUMP_IF_TRUE => -1,
            TargetOpcode::JUMP_IF_FALSE_OR_POP | TargetOpcode::JUMP_IF_TRUE_OR_POP => {
                panic!("JUMP_IF_*_OR_POP requires runtime info");
            }
            TargetOpcode::FOR_ITER => 1,
            TargetOpcode::LOAD_GLOBAL => 1,
            TargetOpcode::SETUP_LOOP => 0,
            TargetOpcode::SETUP_EXCEPT | TargetOpcode::SETUP_FINALLY => {
                panic!("SETUP_EXCEPT requires runtime info");
            }
            TargetOpcode::STORE_MAP => -2,
            TargetOpcode::LOAD_FAST => 1,
            TargetOpcode::DELETE_FAST => 0,
            TargetOpcode::LOAD_CLOSURE => 0,
            TargetOpcode::LOAD_DEREF => 1,
            TargetOpcode::RAISE_VARARGS => 0,
            TargetOpcode::CALL_FUNCTION => {
                let pos_args = self.arg.unwrap() & 0xFF;
                let kwargs = (self.arg.unwrap() >> 8) & 0xFF;
                // 1 arg is removed for the callable, 1 is added for the return value
                -(pos_args as isize + kwargs as isize + 1) + 1
            }
            TargetOpcode::MAKE_FUNCTION => -1,
            TargetOpcode::MAKE_CLOSURE => 2 + self.arg.unwrap() as isize,
            TargetOpcode::BUILD_SLICE => 1 - (self.arg.unwrap() as isize),
            TargetOpcode::EXTENDED_ARG => panic!("not supported yet"),
            TargetOpcode::CALL_FUNCTION_VAR | TargetOpcode::CALL_FUNCTION_KW => {
                let pos_args = self.arg.unwrap() & 0xFF;
                let kwargs = (self.arg.unwrap() >> 8) & 0xFF;
                // 1 arg is removed for the callable and additional positional args, 1 is added for the return value
                -(pos_args as isize + kwargs as isize + 2) + 1
            }
            TargetOpcode::CALL_FUNCTION_VAR_KW => {
                let pos_args = self.arg.unwrap() & 0xFF;
                let kwargs = (self.arg.unwrap() >> 8) & 0xFF;
                // 1 arg is removed for the callable and additional positional+kw args, 1 is added for the return value
                -(pos_args as isize + kwargs as isize + 3) + 1
            }
        }
    }
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

    /// Whether or not this opcode is any kind of instruction which may jump
    fn is_jump(&self) -> bool {
        self.is_relative_jump() || self.is_absolute_jump()
    }

    fn is_other_conditional_jump(&self) -> bool;

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
