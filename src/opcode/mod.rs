use std::fmt::{self, Debug};

pub use num_traits::FromPrimitive;
pub use num_traits::ToPrimitive;

use self::py27::Mnemonic;

pub mod py27;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction<O: Opcode> {
    pub opcode: O,
    pub arg: Option<u16>,
}

#[macro_export]
macro_rules! Instr {
    ($opcode:expr) => {
        ::pydis::opcode::Instruction {
            opcode: $opcode,
            arg: None,
        }
    };
    ($opcode:expr, $arg:expr) => {
        ::pydis::opcode::Instruction {
            opcode: $opcode,
            arg: Some($arg),
        }
    };
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

impl<O: Opcode<Mnemonic = py27::Mnemonic>> Instruction<O> {
    /// How the stack adjusts after this instruction executes. A positive number indicates that N
    /// elements were pushed to the stack while a negative number indicates the number of elements
    /// removed from the stack
    pub fn stack_adjustment_after(&self) -> isize {
        match self.opcode.mnemonic() {
            // Meta instructions
            Mnemonic::STOP_CODE
            | Mnemonic::NOP
            | Mnemonic::ROT_TWO
            | Mnemonic::ROT_THREE
            | Mnemonic::ROT_FOUR => 0,

            Mnemonic::POP_TOP => -1,
            Mnemonic::DUP_TOP => 1,
            Mnemonic::DUP_TOP_TWO => 2,
            // Unary ops
            Mnemonic::UNARY_POSITIVE
            | Mnemonic::UNARY_NEGATIVE
            | Mnemonic::UNARY_NOT
            | Mnemonic::UNARY_CONVERT
            | Mnemonic::UNARY_INVERT
            | Mnemonic::GET_ITER => 0,
            // Binary ops
            Mnemonic::BINARY_POWER
            | Mnemonic::BINARY_MULTIPLY
            | Mnemonic::BINARY_DIVIDE
            | Mnemonic::BINARY_FLOOR_DIVIDE
            | Mnemonic::BINARY_TRUE_DIVIDE
            | Mnemonic::BINARY_MODULO
            | Mnemonic::BINARY_ADD
            | Mnemonic::BINARY_SUBTRACT
            | Mnemonic::BINARY_SUBSC
            | Mnemonic::BINARY_LSHIFT
            | Mnemonic::BINARY_RSHIFT
            | Mnemonic::BINARY_AND
            | Mnemonic::BINARY_XOR
            | Mnemonic::BINARY_OR => -1,
            // In-place operations
            Mnemonic::INPLACE_POWER
            | Mnemonic::INPLACE_MULTIPLY
            | Mnemonic::INPLACE_DIVIDE
            | Mnemonic::INPLACE_FLOOR_DIVIDE
            | Mnemonic::INPLACE_TRUE_DIVIDE
            | Mnemonic::INPLACE_MODULO
            | Mnemonic::INPLACE_ADD
            | Mnemonic::INPLACE_SUBTRACT
            | Mnemonic::INPLACE_LSHIFT
            | Mnemonic::INPLACE_RSHIFT
            | Mnemonic::INPLACE_AND
            | Mnemonic::INPLACE_XOR
            | Mnemonic::INPLACE_OR => -1,
            // Slice operations
            Mnemonic::SLICE_0 => 0,
            Mnemonic::SLICE_1 => -1,
            Mnemonic::SLICE_2 => -2,
            Mnemonic::SLICE_3 => -3,
            Mnemonic::STORE_SLICE_0 => -1,
            Mnemonic::STORE_SLICE_1 => -2,
            Mnemonic::STORE_SLICE_2 => -3,
            Mnemonic::STORE_SLICE_3 => -4,
            Mnemonic::DELETE_SLICE_0 => -1,
            Mnemonic::DELETE_SLICE_1 => -2,
            Mnemonic::DELETE_SLICE_2 => -3,
            Mnemonic::DELETE_SLICE_3 => -4,
            Mnemonic::STORE_SUBSCR => -3,
            Mnemonic::DELETE_SUBSCR => -2,
            // Misc
            Mnemonic::PRINT_EXPR => -1,
            Mnemonic::PRINT_ITEM => -1,
            Mnemonic::PRINT_ITEM_TO => -2,
            Mnemonic::PRINT_NEWLINE => 0,
            Mnemonic::PRINT_NEWLINE_TO => -1,
            Mnemonic::BREAK_LOOP => 0,
            Mnemonic::CONTINUE_LOOP => 0,
            Mnemonic::LIST_APPEND => -1,
            Mnemonic::LOAD_LOCALS => 1,
            Mnemonic::RETURN_VALUE => 0,
            Mnemonic::YIELD_VALUE => 0,
            Mnemonic::IMPORT_STAR => -1,
            Mnemonic::EXEC_STMT => -3,
            Mnemonic::POP_BLOCK => 0,
            Mnemonic::END_FINALLY => 0,
            Mnemonic::BUILD_CLASS => -3,
            // TODO: maybe not right?
            Mnemonic::SETUP_WITH => 1,
            Mnemonic::WITH_CLEANUP => {
                panic!("with_cleanup may require runtime info");
            }
            Mnemonic::STORE_NAME => -1,
            Mnemonic::STORE_FAST => -1,
            Mnemonic::STORE_DEREF => -1,
            Mnemonic::SET_ADD => -1,
            Mnemonic::MAP_ADD => -1,
            Mnemonic::DELETE_NAME => 0,
            Mnemonic::UNPACK_SEQUENCE => (self.arg.unwrap() as isize) - 1,
            Mnemonic::DUP_TOPX => self.arg.unwrap() as isize,
            Mnemonic::STORE_ATTR => -2,
            Mnemonic::DELETE_ATTR => -1,
            Mnemonic::STORE_GLOBAL => -1,
            Mnemonic::DELETE_GLOBAL => 0,
            Mnemonic::LOAD_CONST => 1,
            Mnemonic::LOAD_NAME => 1,
            Mnemonic::BUILD_TUPLE | Mnemonic::BUILD_LIST | Mnemonic::BUILD_SET => {
                (self.arg.unwrap() as isize) - 1
            }
            Mnemonic::BUILD_MAP => 1,
            Mnemonic::LOAD_ATTR => 0,
            Mnemonic::COMPARE_OP => -1,
            Mnemonic::IMPORT_NAME => -1,
            Mnemonic::IMPORT_FROM => 1,
            Mnemonic::JUMP_FORWARD | Mnemonic::JUMP_ABSOLUTE => 0,
            Mnemonic::POP_JUMP_IF_FALSE | Mnemonic::POP_JUMP_IF_TRUE => -1,
            Mnemonic::JUMP_IF_FALSE_OR_POP | Mnemonic::JUMP_IF_TRUE_OR_POP => {
                panic!("JUMP_IF_*_OR_POP requires runtime info");
            }
            Mnemonic::FOR_ITER => 1,
            Mnemonic::LOAD_GLOBAL => 1,
            Mnemonic::SETUP_LOOP => 0,
            Mnemonic::SETUP_EXCEPT | Mnemonic::SETUP_FINALLY => {
                panic!("SETUP_EXCEPT requires runtime info");
            }
            Mnemonic::STORE_MAP => -2,
            Mnemonic::LOAD_FAST => 1,
            Mnemonic::DELETE_FAST => 0,
            Mnemonic::LOAD_CLOSURE => 0,
            Mnemonic::LOAD_DEREF => 1,
            Mnemonic::RAISE_VARARGS => 0,
            Mnemonic::CALL_FUNCTION => {
                let pos_args = self.arg.unwrap() & 0xFF;
                let kwargs = (self.arg.unwrap() >> 8) & 0xFF;
                // 1 arg is removed for the callable, 1 is added for the return value
                -(pos_args as isize + kwargs as isize + 1) + 1
            }
            Mnemonic::MAKE_FUNCTION => -1,
            Mnemonic::MAKE_CLOSURE => 2 + self.arg.unwrap() as isize,
            Mnemonic::BUILD_SLICE => 1 - (self.arg.unwrap() as isize),
            Mnemonic::EXTENDED_ARG => panic!("not supported yet"),
            Mnemonic::CALL_FUNCTION_VAR | Mnemonic::CALL_FUNCTION_KW => {
                let pos_args = self.arg.unwrap() & 0xFF;
                let kwargs = (self.arg.unwrap() >> 8) & 0xFF;
                // 1 arg is removed for the callable and additional positional args, 1 is added for the return value
                -(pos_args as isize + kwargs as isize + 2) + 1
            }
            Mnemonic::CALL_FUNCTION_VAR_KW => {
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
pub trait Opcode: From<Self::Mnemonic> + Send + Sync + FromPrimitive + ToPrimitive + Copy + Clone + Debug {
    type Mnemonic;

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

    fn mnemonic(&self) -> Self::Mnemonic;
}
