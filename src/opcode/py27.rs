pub use enum_primitive_derive::Primitive;

/// Standard set of instruction mnemoics
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mnemonic {
    STOP_CODE,
    POP_TOP,
    ROT_TWO,
    ROT_THREE,
    DUP_TOP,
    DUP_TOP_TWO,
    ROT_FOUR,

    NOP,
    UNARY_POSITIVE,
    UNARY_NEGATIVE,
    UNARY_NOT,
    UNARY_CONVERT,

    UNARY_INVERT,

    BINARY_POWER,
    BINARY_MULTIPLY,
    BINARY_DIVIDE,
    BINARY_MODULO,
    BINARY_ADD,
    BINARY_SUBTRACT,
    BINARY_SUBSC,
    BINARY_FLOOR_DIVIDE,
    BINARY_TRUE_DIVIDE,
    INPLACE_FLOOR_DIVIDE,
    INPLACE_TRUE_DIVIDE,
    SLICE_0,
    SLICE_1,
    SLICE_2,
    SLICE_3,

    STORE_SLICE_0,
    STORE_SLICE_1,
    STORE_SLICE_2,
    STORE_SLICE_3,

    DELETE_SLICE_0,
    DELETE_SLICE_1,
    DELETE_SLICE_2,
    DELETE_SLICE_3,

    STORE_MAP,
    INPLACE_ADD,
    INPLACE_SUBTRACT,
    INPLACE_MULTIPLY,
    INPLACE_DIVIDE,
    INPLACE_MODULO,
    STORE_SUBSCR,
    DELETE_SUBSCR,
    BINARY_LSHIFT,
    BINARY_RSHIFT,
    BINARY_AND,
    BINARY_XOR,
    BINARY_OR,
    INPLACE_POWER,
    GET_ITER,

    PRINT_EXPR,
    PRINT_ITEM,
    PRINT_NEWLINE,
    PRINT_ITEM_TO,
    PRINT_NEWLINE_TO,
    INPLACE_LSHIFT,
    INPLACE_RSHIFT,
    INPLACE_AND,
    INPLACE_XOR,
    INPLACE_OR,
    BREAK_LOOP,
    WITH_CLEANUP,
    LOAD_LOCALS,
    RETURN_VALUE,
    IMPORT_STAR,
    EXEC_STMT,
    YIELD_VALUE,
    POP_BLOCK,
    END_FINALLY,
    BUILD_CLASS,

    // Opcodes with arguments
    STORE_NAME,
    DELETE_NAME,
    UNPACK_SEQUENCE,
    FOR_ITER,
    LIST_APPEND,
    STORE_ATTR,
    DELETE_ATTR,
    STORE_GLOBAL,
    DELETE_GLOBAL,
    DUP_TOPX,
    LOAD_CONST,
    LOAD_NAME,
    BUILD_TUPLE,
    BUILD_LIST,
    BUILD_SET,
    BUILD_MAP,
    LOAD_ATTR,
    COMPARE_OP,
    IMPORT_NAME,
    IMPORT_FROM,
    JUMP_FORWARD,
    JUMP_IF_FALSE_OR_POP,
    JUMP_IF_TRUE_OR_POP,
    JUMP_ABSOLUTE,
    POP_JUMP_IF_FALSE,
    POP_JUMP_IF_TRUE,
    LOAD_GLOBAL,

    CONTINUE_LOOP,
    SETUP_LOOP,
    SETUP_EXCEPT,
    SETUP_FINALLY,

    LOAD_FAST,
    STORE_FAST,
    DELETE_FAST,

    RAISE_VARARGS,
    CALL_FUNCTION,
    MAKE_FUNCTION,
    BUILD_SLICE,
    MAKE_CLOSURE,
    LOAD_CLOSURE,
    LOAD_DEREF,
    STORE_DEREF,

    CALL_FUNCTION_VAR,
    CALL_FUNCTION_KW,
    CALL_FUNCTION_VAR_KW,

    SETUP_WITH,
    EXTENDED_ARG,
    SET_ADD,
    MAP_ADD,
}

/// Opcodes taken from https://github.com/python/cpython/blob/2.7/Lib/opcode.py.
/// This is the standard VM opcode set.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Primitive)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Standard {
    STOP_CODE = 0,
    POP_TOP = 1,
    ROT_TWO = 2,
    ROT_THREE = 3,
    DUP_TOP = 4,
    DUP_TOP_TWO = 5,
    ROT_FOUR = 6,

    NOP = 9,
    UNARY_POSITIVE = 10,
    UNARY_NEGATIVE = 11,
    UNARY_NOT = 12,
    UNARY_CONVERT = 13,

    UNARY_INVERT = 15,

    BINARY_POWER = 19,
    BINARY_MULTIPLY = 20,
    BINARY_DIVIDE = 21,
    BINARY_MODULO = 22,
    BINARY_ADD = 23,
    BINARY_SUBTRACT = 24,
    BINARY_SUBSC = 25,
    BINARY_FLOOR_DIVIDE = 26,
    BINARY_TRUE_DIVIDE = 27,
    INPLACE_FLOOR_DIVIDE = 28,
    INPLACE_TRUE_DIVIDE = 29,
    SLICE_0 = 30,
    SLICE_1 = 31,
    SLICE_2 = 32,
    SLICE_3 = 33,

    STORE_SLICE_0 = 40,
    STORE_SLICE_1 = 41,
    STORE_SLICE_2 = 42,
    STORE_SLICE_3 = 43,

    DELETE_SLICE_0 = 50,
    DELETE_SLICE_1 = 51,
    DELETE_SLICE_2 = 52,
    DELETE_SLICE_3 = 53,

    STORE_MAP = 54,
    INPLACE_ADD = 55,
    INPLACE_SUBTRACT = 56,
    INPLACE_MULTIPLY = 57,
    INPLACE_DIVIDE = 58,
    INPLACE_MODULO = 59,
    STORE_SUBSCR = 60,
    DELETE_SUBSCR = 61,
    BINARY_LSHIFT = 62,
    BINARY_RSHIFT = 63,
    BINARY_AND = 64,
    BINARY_XOR = 65,
    BINARY_OR = 66,
    INPLACE_POWER = 67,
    GET_ITER = 68,

    PRINT_EXPR = 70,
    PRINT_ITEM = 71,
    PRINT_NEWLINE = 72,
    PRINT_ITEM_TO = 73,
    PRINT_NEWLINE_TO = 74,
    INPLACE_LSHIFT = 75,
    INPLACE_RSHIFT = 76,
    INPLACE_AND = 77,
    INPLACE_XOR = 78,
    INPLACE_OR = 79,
    BREAK_LOOP = 80,
    WITH_CLEANUP = 81,
    LOAD_LOCALS = 82,
    RETURN_VALUE = 83,
    IMPORT_STAR = 84,
    EXEC_STMT = 85,
    YIELD_VALUE = 86,
    POP_BLOCK = 87,
    END_FINALLY = 88,
    BUILD_CLASS = 89,

    // Opcodes with arguments
    STORE_NAME = 90,
    DELETE_NAME = 91,
    UNPACK_SEQUENCE = 92,
    FOR_ITER = 93,
    LIST_APPEND = 94,
    STORE_ATTR = 95,
    DELETE_ATTR = 96,
    STORE_GLOBAL = 97,
    DELETE_GLOBAL = 98,
    DUP_TOPX = 99,
    LOAD_CONST = 100,
    LOAD_NAME = 101,
    BUILD_TUPLE = 102,
    BUILD_LIST = 103,
    BUILD_SET = 104,
    BUILD_MAP = 105,
    LOAD_ATTR = 106,
    COMPARE_OP = 107,
    IMPORT_NAME = 108,
    IMPORT_FROM = 109,
    JUMP_FORWARD = 110,
    JUMP_IF_FALSE_OR_POP = 111,
    JUMP_IF_TRUE_OR_POP = 112,
    JUMP_ABSOLUTE = 113,
    POP_JUMP_IF_FALSE = 114,
    POP_JUMP_IF_TRUE = 115,
    LOAD_GLOBAL = 116,

    CONTINUE_LOOP = 119,
    SETUP_LOOP = 120,
    SETUP_EXCEPT = 121,
    SETUP_FINALLY = 122,

    LOAD_FAST = 124,
    STORE_FAST = 125,
    DELETE_FAST = 126,

    RAISE_VARARGS = 130,
    CALL_FUNCTION = 131,
    MAKE_FUNCTION = 132,
    BUILD_SLICE = 133,
    MAKE_CLOSURE = 134,
    LOAD_CLOSURE = 135,
    LOAD_DEREF = 136,
    STORE_DEREF = 138,

    CALL_FUNCTION_VAR = 140,
    CALL_FUNCTION_KW = 141,
    CALL_FUNCTION_VAR_KW = 142,

    SETUP_WITH = 143,
    EXTENDED_ARG = 145,
    SET_ADD = 146,
    MAP_ADD = 147,
}

impl From<Mnemonic> for Standard {
    fn from(mnemonic: Mnemonic) -> Self {
        match mnemonic {
            Mnemonic::STOP_CODE => Self::STOP_CODE,
            Mnemonic::POP_TOP => Self::POP_TOP,
            Mnemonic::ROT_TWO => Self::ROT_TWO,
            Mnemonic::ROT_THREE => Self::ROT_THREE,
            Mnemonic::DUP_TOP => Self::DUP_TOP,
            Mnemonic::DUP_TOP_TWO => Self::DUP_TOP_TWO,
            Mnemonic::ROT_FOUR => Self::ROT_FOUR,

            Mnemonic::NOP => Self::NOP,
            Mnemonic::UNARY_POSITIVE => Self::UNARY_POSITIVE,
            Mnemonic::UNARY_NEGATIVE => Self::UNARY_NEGATIVE,
            Mnemonic::UNARY_NOT => Self::UNARY_NOT,
            Mnemonic::UNARY_CONVERT => Self::UNARY_CONVERT,

            Mnemonic::UNARY_INVERT => Self::UNARY_INVERT,

            Mnemonic::BINARY_POWER => Self::BINARY_POWER,
            Mnemonic::BINARY_MULTIPLY => Self::BINARY_MULTIPLY,
            Mnemonic::BINARY_DIVIDE => Self::BINARY_DIVIDE,
            Mnemonic::BINARY_MODULO => Self::BINARY_MODULO,
            Mnemonic::BINARY_ADD => Self::BINARY_ADD,
            Mnemonic::BINARY_SUBTRACT => Self::BINARY_SUBTRACT,
            Mnemonic::BINARY_SUBSC => Self::BINARY_SUBSC,
            Mnemonic::BINARY_FLOOR_DIVIDE => Self::BINARY_FLOOR_DIVIDE,
            Mnemonic::BINARY_TRUE_DIVIDE => Self::BINARY_TRUE_DIVIDE,
            Mnemonic::INPLACE_FLOOR_DIVIDE => Self::INPLACE_FLOOR_DIVIDE,
            Mnemonic::INPLACE_TRUE_DIVIDE => Self::INPLACE_TRUE_DIVIDE,
            Mnemonic::SLICE_0 => Self::SLICE_0,
            Mnemonic::SLICE_1 => Self::SLICE_1,
            Mnemonic::SLICE_2 => Self::SLICE_2,
            Mnemonic::SLICE_3 => Self::SLICE_3,

            Mnemonic::STORE_SLICE_0 => Self::STORE_SLICE_0,
            Mnemonic::STORE_SLICE_1 => Self::STORE_SLICE_1,
            Mnemonic::STORE_SLICE_2 => Self::STORE_SLICE_2,
            Mnemonic::STORE_SLICE_3 => Self::STORE_SLICE_3,

            Mnemonic::DELETE_SLICE_0 => Self::DELETE_SLICE_0,
            Mnemonic::DELETE_SLICE_1 => Self::DELETE_SLICE_1,
            Mnemonic::DELETE_SLICE_2 => Self::DELETE_SLICE_2,
            Mnemonic::DELETE_SLICE_3 => Self::DELETE_SLICE_3,

            Mnemonic::STORE_MAP => Self::STORE_MAP,
            Mnemonic::INPLACE_ADD => Self::INPLACE_ADD,
            Mnemonic::INPLACE_SUBTRACT => Self::INPLACE_SUBTRACT,
            Mnemonic::INPLACE_MULTIPLY => Self::INPLACE_MULTIPLY,
            Mnemonic::INPLACE_DIVIDE => Self::INPLACE_DIVIDE,
            Mnemonic::INPLACE_MODULO => Self::INPLACE_MODULO,
            Mnemonic::STORE_SUBSCR => Self::STORE_SUBSCR,
            Mnemonic::DELETE_SUBSCR => Self::DELETE_SUBSCR,
            Mnemonic::BINARY_LSHIFT => Self::BINARY_LSHIFT,
            Mnemonic::BINARY_RSHIFT => Self::BINARY_RSHIFT,
            Mnemonic::BINARY_AND => Self::BINARY_AND,
            Mnemonic::BINARY_XOR => Self::BINARY_XOR,
            Mnemonic::BINARY_OR => Self::BINARY_OR,
            Mnemonic::INPLACE_POWER => Self::INPLACE_POWER,
            Mnemonic::GET_ITER => Self::GET_ITER,

            Mnemonic::PRINT_EXPR => Self::PRINT_EXPR,
            Mnemonic::PRINT_ITEM => Self::PRINT_ITEM,
            Mnemonic::PRINT_NEWLINE => Self::PRINT_NEWLINE,
            Mnemonic::PRINT_ITEM_TO => Self::PRINT_ITEM_TO,
            Mnemonic::PRINT_NEWLINE_TO => Self::PRINT_NEWLINE_TO,
            Mnemonic::INPLACE_LSHIFT => Self::INPLACE_LSHIFT,
            Mnemonic::INPLACE_RSHIFT => Self::INPLACE_RSHIFT,
            Mnemonic::INPLACE_AND => Self::INPLACE_AND,
            Mnemonic::INPLACE_XOR => Self::INPLACE_XOR,
            Mnemonic::INPLACE_OR => Self::INPLACE_OR,
            Mnemonic::BREAK_LOOP => Self::BREAK_LOOP,
            Mnemonic::WITH_CLEANUP => Self::WITH_CLEANUP,
            Mnemonic::LOAD_LOCALS => Self::LOAD_LOCALS,
            Mnemonic::RETURN_VALUE => Self::RETURN_VALUE,
            Mnemonic::IMPORT_STAR => Self::IMPORT_STAR,
            Mnemonic::EXEC_STMT => Self::EXEC_STMT,
            Mnemonic::YIELD_VALUE => Self::YIELD_VALUE,
            Mnemonic::POP_BLOCK => Self::POP_BLOCK,
            Mnemonic::END_FINALLY => Self::END_FINALLY,
            Mnemonic::BUILD_CLASS => Self::BUILD_CLASS,

            //Mnemonic::Opcodes => Self::Opcodes,
            Mnemonic::STORE_NAME => Self::STORE_NAME,
            Mnemonic::DELETE_NAME => Self::DELETE_NAME,
            Mnemonic::UNPACK_SEQUENCE => Self::UNPACK_SEQUENCE,
            Mnemonic::FOR_ITER => Self::FOR_ITER,
            Mnemonic::LIST_APPEND => Self::LIST_APPEND,
            Mnemonic::STORE_ATTR => Self::STORE_ATTR,
            Mnemonic::DELETE_ATTR => Self::DELETE_ATTR,
            Mnemonic::STORE_GLOBAL => Self::STORE_GLOBAL,
            Mnemonic::DELETE_GLOBAL => Self::DELETE_GLOBAL,
            Mnemonic::DUP_TOPX => Self::DUP_TOPX,
            Mnemonic::LOAD_CONST => Self::LOAD_CONST,
            Mnemonic::LOAD_NAME => Self::LOAD_NAME,
            Mnemonic::BUILD_TUPLE => Self::BUILD_TUPLE,
            Mnemonic::BUILD_LIST => Self::BUILD_LIST,
            Mnemonic::BUILD_SET => Self::BUILD_SET,
            Mnemonic::BUILD_MAP => Self::BUILD_MAP,
            Mnemonic::LOAD_ATTR => Self::LOAD_ATTR,
            Mnemonic::COMPARE_OP => Self::COMPARE_OP,
            Mnemonic::IMPORT_NAME => Self::IMPORT_NAME,
            Mnemonic::IMPORT_FROM => Self::IMPORT_FROM,
            Mnemonic::JUMP_FORWARD => Self::JUMP_FORWARD,
            Mnemonic::JUMP_IF_FALSE_OR_POP => Self::JUMP_IF_FALSE_OR_POP,
            Mnemonic::JUMP_IF_TRUE_OR_POP => Self::JUMP_IF_TRUE_OR_POP,
            Mnemonic::JUMP_ABSOLUTE => Self::JUMP_ABSOLUTE,
            Mnemonic::POP_JUMP_IF_FALSE => Self::POP_JUMP_IF_FALSE,
            Mnemonic::POP_JUMP_IF_TRUE => Self::POP_JUMP_IF_TRUE,
            Mnemonic::LOAD_GLOBAL => Self::LOAD_GLOBAL,

            Mnemonic::CONTINUE_LOOP => Self::CONTINUE_LOOP,
            Mnemonic::SETUP_LOOP => Self::SETUP_LOOP,
            Mnemonic::SETUP_EXCEPT => Self::SETUP_EXCEPT,
            Mnemonic::SETUP_FINALLY => Self::SETUP_FINALLY,

            Mnemonic::LOAD_FAST => Self::LOAD_FAST,
            Mnemonic::STORE_FAST => Self::STORE_FAST,
            Mnemonic::DELETE_FAST => Self::DELETE_FAST,

            Mnemonic::RAISE_VARARGS => Self::RAISE_VARARGS,
            Mnemonic::CALL_FUNCTION => Self::CALL_FUNCTION,
            Mnemonic::MAKE_FUNCTION => Self::MAKE_FUNCTION,
            Mnemonic::BUILD_SLICE => Self::BUILD_SLICE,
            Mnemonic::MAKE_CLOSURE => Self::MAKE_CLOSURE,
            Mnemonic::LOAD_CLOSURE => Self::LOAD_CLOSURE,
            Mnemonic::LOAD_DEREF => Self::LOAD_DEREF,
            Mnemonic::STORE_DEREF => Self::STORE_DEREF,

            Mnemonic::CALL_FUNCTION_VAR => Self::CALL_FUNCTION_VAR,
            Mnemonic::CALL_FUNCTION_KW => Self::CALL_FUNCTION_KW,
            Mnemonic::CALL_FUNCTION_VAR_KW => Self::CALL_FUNCTION_VAR_KW,

            Mnemonic::SETUP_WITH => Self::SETUP_WITH,
            Mnemonic::EXTENDED_ARG => Self::EXTENDED_ARG,
            Mnemonic::SET_ADD => Self::SET_ADD,
            Mnemonic::MAP_ADD => Self::MAP_ADD,
        }
    }
}

impl super::Opcode for Standard {
    type Mnemonic = Mnemonic;

    /// Whether or not this opcode has an argument
    fn has_arg(&self) -> bool {
        *self as u8 >= 90
    }

    /// Whether or not this opcode has an extended argument
    fn has_extended_arg(&self) -> bool {
        *self as u8 >= 144
    }

    /// Whether or not this opcode has a constant parameter
    fn has_const(&self) -> bool {
        *self == Self::LOAD_CONST
    }

    /// Whether or not this opcode is a boolean operation
    fn has_comp(&self) -> bool {
        matches!(self, Self::COMPARE_OP)
    }

    /// Whether or not this opcode has a relative jump target
    fn is_relative_jump(&self) -> bool {
        matches!(
            self,
            Self::FOR_ITER
                | Self::JUMP_FORWARD
                | Self::SETUP_LOOP
                | Self::SETUP_EXCEPT
                | Self::SETUP_FINALLY
                | Self::SETUP_WITH
        )
    }

    /// Whether or not this opcode has an absolute jump target
    fn is_absolute_jump(&self) -> bool {
        matches!(
            self,
            Self::JUMP_IF_FALSE_OR_POP
                | Self::JUMP_IF_TRUE_OR_POP
                | Self::JUMP_ABSOLUTE
                | Self::POP_JUMP_IF_FALSE
                | Self::POP_JUMP_IF_TRUE
                | Self::CONTINUE_LOOP
        )
    }

    /// Whether or not this opcode is another type of "special" jumping instruction
    /// e.g. FOR_ITER, SETUP_LOOP, etc.
    fn is_other_conditional_jump(&self) -> bool {
        matches!(
            self,
            Self::FOR_ITER
                | Self::SETUP_LOOP
                | Self::SETUP_EXCEPT
                | Self::SETUP_FINALLY
                | Self::SETUP_WITH
        )
    }

    /// Whether or not this opcode is a conditional jump
    fn is_conditional_jump(&self) -> bool {
        matches!(
            self,
            Self::JUMP_IF_FALSE_OR_POP
                | Self::JUMP_IF_TRUE_OR_POP
                | Self::POP_JUMP_IF_FALSE
                | Self::POP_JUMP_IF_TRUE
        )
    }

    /// Whether or not this opcode accesses an attribute by name
    fn has_name(&self) -> bool {
        matches!(
            self,
            Self::STORE_NAME
                | Self::DELETE_NAME
                | Self::STORE_ATTR
                | Self::DELETE_ATTR
                | Self::STORE_GLOBAL
                | Self::DELETE_GLOBAL
                | Self::LOAD_NAME
                | Self::LOAD_ATTR
                | Self::IMPORT_NAME
                | Self::IMPORT_FROM
                | Self::LOAD_GLOBAL
        )
    }

    /// Whether or not this opcode accesses a local variable
    fn has_local(&self) -> bool {
        matches!(self, Self::LOAD_FAST | Self::STORE_FAST | Self::DELETE_FAST)
    }

    /// Whether or not this opcode accesses a free variable
    fn has_free(&self) -> bool {
        matches!(
            self,
            Self::LOAD_CLOSURE | Self::LOAD_DEREF | Self::STORE_DEREF
        )
    }

    fn mnemonic(&self) -> Mnemonic {
        match self {
            Self::STOP_CODE => Mnemonic::STOP_CODE,
            Self::POP_TOP => Mnemonic::POP_TOP,
            Self::ROT_TWO => Mnemonic::ROT_TWO,
            Self::ROT_THREE => Mnemonic::ROT_THREE,
            Self::DUP_TOP => Mnemonic::DUP_TOP,
            Self::DUP_TOP_TWO => Mnemonic::DUP_TOP_TWO,
            Self::ROT_FOUR => Mnemonic::ROT_FOUR,

            Self::NOP => Mnemonic::NOP,
            Self::UNARY_POSITIVE => Mnemonic::UNARY_POSITIVE,
            Self::UNARY_NEGATIVE => Mnemonic::UNARY_NEGATIVE,
            Self::UNARY_NOT => Mnemonic::UNARY_NOT,
            Self::UNARY_CONVERT => Mnemonic::UNARY_CONVERT,

            Self::UNARY_INVERT => Mnemonic::UNARY_INVERT,

            Self::BINARY_POWER => Mnemonic::BINARY_POWER,
            Self::BINARY_MULTIPLY => Mnemonic::BINARY_MULTIPLY,
            Self::BINARY_DIVIDE => Mnemonic::BINARY_DIVIDE,
            Self::BINARY_MODULO => Mnemonic::BINARY_MODULO,
            Self::BINARY_ADD => Mnemonic::BINARY_ADD,
            Self::BINARY_SUBTRACT => Mnemonic::BINARY_SUBTRACT,
            Self::BINARY_SUBSC => Mnemonic::BINARY_SUBSC,
            Self::BINARY_FLOOR_DIVIDE => Mnemonic::BINARY_FLOOR_DIVIDE,
            Self::BINARY_TRUE_DIVIDE => Mnemonic::BINARY_TRUE_DIVIDE,
            Self::INPLACE_FLOOR_DIVIDE => Mnemonic::INPLACE_FLOOR_DIVIDE,
            Self::INPLACE_TRUE_DIVIDE => Mnemonic::INPLACE_TRUE_DIVIDE,
            Self::SLICE_0 => Mnemonic::SLICE_0,
            Self::SLICE_1 => Mnemonic::SLICE_1,
            Self::SLICE_2 => Mnemonic::SLICE_2,
            Self::SLICE_3 => Mnemonic::SLICE_3,

            Self::STORE_SLICE_0 => Mnemonic::STORE_SLICE_0,
            Self::STORE_SLICE_1 => Mnemonic::STORE_SLICE_1,
            Self::STORE_SLICE_2 => Mnemonic::STORE_SLICE_2,
            Self::STORE_SLICE_3 => Mnemonic::STORE_SLICE_3,

            Self::DELETE_SLICE_0 => Mnemonic::DELETE_SLICE_0,
            Self::DELETE_SLICE_1 => Mnemonic::DELETE_SLICE_1,
            Self::DELETE_SLICE_2 => Mnemonic::DELETE_SLICE_2,
            Self::DELETE_SLICE_3 => Mnemonic::DELETE_SLICE_3,

            Self::STORE_MAP => Mnemonic::STORE_MAP,
            Self::INPLACE_ADD => Mnemonic::INPLACE_ADD,
            Self::INPLACE_SUBTRACT => Mnemonic::INPLACE_SUBTRACT,
            Self::INPLACE_MULTIPLY => Mnemonic::INPLACE_MULTIPLY,
            Self::INPLACE_DIVIDE => Mnemonic::INPLACE_DIVIDE,
            Self::INPLACE_MODULO => Mnemonic::INPLACE_MODULO,
            Self::STORE_SUBSCR => Mnemonic::STORE_SUBSCR,
            Self::DELETE_SUBSCR => Mnemonic::DELETE_SUBSCR,
            Self::BINARY_LSHIFT => Mnemonic::BINARY_LSHIFT,
            Self::BINARY_RSHIFT => Mnemonic::BINARY_RSHIFT,
            Self::BINARY_AND => Mnemonic::BINARY_AND,
            Self::BINARY_XOR => Mnemonic::BINARY_XOR,
            Self::BINARY_OR => Mnemonic::BINARY_OR,
            Self::INPLACE_POWER => Mnemonic::INPLACE_POWER,
            Self::GET_ITER => Mnemonic::GET_ITER,

            Self::PRINT_EXPR => Mnemonic::PRINT_EXPR,
            Self::PRINT_ITEM => Mnemonic::PRINT_ITEM,
            Self::PRINT_NEWLINE => Mnemonic::PRINT_NEWLINE,
            Self::PRINT_ITEM_TO => Mnemonic::PRINT_ITEM_TO,
            Self::PRINT_NEWLINE_TO => Mnemonic::PRINT_NEWLINE_TO,
            Self::INPLACE_LSHIFT => Mnemonic::INPLACE_LSHIFT,
            Self::INPLACE_RSHIFT => Mnemonic::INPLACE_RSHIFT,
            Self::INPLACE_AND => Mnemonic::INPLACE_AND,
            Self::INPLACE_XOR => Mnemonic::INPLACE_XOR,
            Self::INPLACE_OR => Mnemonic::INPLACE_OR,
            Self::BREAK_LOOP => Mnemonic::BREAK_LOOP,
            Self::WITH_CLEANUP => Mnemonic::WITH_CLEANUP,
            Self::LOAD_LOCALS => Mnemonic::LOAD_LOCALS,
            Self::RETURN_VALUE => Mnemonic::RETURN_VALUE,
            Self::IMPORT_STAR => Mnemonic::IMPORT_STAR,
            Self::EXEC_STMT => Mnemonic::EXEC_STMT,
            Self::YIELD_VALUE => Mnemonic::YIELD_VALUE,
            Self::POP_BLOCK => Mnemonic::POP_BLOCK,
            Self::END_FINALLY => Mnemonic::END_FINALLY,
            Self::BUILD_CLASS => Mnemonic::BUILD_CLASS,

            //Self::Opcodes => Mnemonic::Opcodes,
            Self::STORE_NAME => Mnemonic::STORE_NAME,
            Self::DELETE_NAME => Mnemonic::DELETE_NAME,
            Self::UNPACK_SEQUENCE => Mnemonic::UNPACK_SEQUENCE,
            Self::FOR_ITER => Mnemonic::FOR_ITER,
            Self::LIST_APPEND => Mnemonic::LIST_APPEND,
            Self::STORE_ATTR => Mnemonic::STORE_ATTR,
            Self::DELETE_ATTR => Mnemonic::DELETE_ATTR,
            Self::STORE_GLOBAL => Mnemonic::STORE_GLOBAL,
            Self::DELETE_GLOBAL => Mnemonic::DELETE_GLOBAL,
            Self::DUP_TOPX => Mnemonic::DUP_TOPX,
            Self::LOAD_CONST => Mnemonic::LOAD_CONST,
            Self::LOAD_NAME => Mnemonic::LOAD_NAME,
            Self::BUILD_TUPLE => Mnemonic::BUILD_TUPLE,
            Self::BUILD_LIST => Mnemonic::BUILD_LIST,
            Self::BUILD_SET => Mnemonic::BUILD_SET,
            Self::BUILD_MAP => Mnemonic::BUILD_MAP,
            Self::LOAD_ATTR => Mnemonic::LOAD_ATTR,
            Self::COMPARE_OP => Mnemonic::COMPARE_OP,
            Self::IMPORT_NAME => Mnemonic::IMPORT_NAME,
            Self::IMPORT_FROM => Mnemonic::IMPORT_FROM,
            Self::JUMP_FORWARD => Mnemonic::JUMP_FORWARD,
            Self::JUMP_IF_FALSE_OR_POP => Mnemonic::JUMP_IF_FALSE_OR_POP,
            Self::JUMP_IF_TRUE_OR_POP => Mnemonic::JUMP_IF_TRUE_OR_POP,
            Self::JUMP_ABSOLUTE => Mnemonic::JUMP_ABSOLUTE,
            Self::POP_JUMP_IF_FALSE => Mnemonic::POP_JUMP_IF_FALSE,
            Self::POP_JUMP_IF_TRUE => Mnemonic::POP_JUMP_IF_TRUE,
            Self::LOAD_GLOBAL => Mnemonic::LOAD_GLOBAL,

            Self::CONTINUE_LOOP => Mnemonic::CONTINUE_LOOP,
            Self::SETUP_LOOP => Mnemonic::SETUP_LOOP,
            Self::SETUP_EXCEPT => Mnemonic::SETUP_EXCEPT,
            Self::SETUP_FINALLY => Mnemonic::SETUP_FINALLY,

            Self::LOAD_FAST => Mnemonic::LOAD_FAST,
            Self::STORE_FAST => Mnemonic::STORE_FAST,
            Self::DELETE_FAST => Mnemonic::DELETE_FAST,

            Self::RAISE_VARARGS => Mnemonic::RAISE_VARARGS,
            Self::CALL_FUNCTION => Mnemonic::CALL_FUNCTION,
            Self::MAKE_FUNCTION => Mnemonic::MAKE_FUNCTION,
            Self::BUILD_SLICE => Mnemonic::BUILD_SLICE,
            Self::MAKE_CLOSURE => Mnemonic::MAKE_CLOSURE,
            Self::LOAD_CLOSURE => Mnemonic::LOAD_CLOSURE,
            Self::LOAD_DEREF => Mnemonic::LOAD_DEREF,
            Self::STORE_DEREF => Mnemonic::STORE_DEREF,

            Self::CALL_FUNCTION_VAR => Mnemonic::CALL_FUNCTION_VAR,
            Self::CALL_FUNCTION_KW => Mnemonic::CALL_FUNCTION_KW,
            Self::CALL_FUNCTION_VAR_KW => Mnemonic::CALL_FUNCTION_VAR_KW,

            Self::SETUP_WITH => Mnemonic::SETUP_WITH,
            Self::EXTENDED_ARG => Mnemonic::EXTENDED_ARG,
            Self::SET_ADD => Mnemonic::SET_ADD,
            Self::MAP_ADD => Mnemonic::MAP_ADD,
        }
    }
}
