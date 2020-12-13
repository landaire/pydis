use enum_primitive_derive::Primitive;

/// Opcodes taken from https://github.com/python/cpython/blob/2.7/Lib/opcode.py
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Primitive)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Python27 {
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
    UNPACK_SEUQNECE = 92,
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

impl super::Opcode for Python27 {
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

    /// Whether or not this opcode is a conditional jump
    fn is_conditional_jump(&self) -> bool {
        matches!(
            self,
            Self::JUMP_IF_FALSE_OR_POP
                | Self::JUMP_IF_TRUE_OR_POP
                | Self::JUMP_ABSOLUTE
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
}
