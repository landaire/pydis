use enum_primitive_derive::Primitive;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub arg: Option<u16>,
}

/// Opcodes taken from https://github.com/python/cpython/blob/2.7/Lib/opcode.py
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Primitive)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Opcode {
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
    UNARY_INVERT = 15,
    BINARY_MATRIX_MULTIPLY = 16,
    INPLACE_MATRIX_MULTIPLY = 17,
    BINARY_POWER = 19,
    BINARY_MULTIPLY = 20,
    BINARY_MODULO = 22,
    BINARY_ADD = 23,
    BINARY_SUBTRACT = 24,
    BINARY_SUBSC = 25,
    BINARY_FLOOR_DIVIDE = 26,
    BINARY_TRUE_DIVIDE = 27,
    INPLACE_FLOOR_DIVIDE = 28,
    INPLACE_TRUE_DIVIDE = 29,
    RERAISE = 48,
    WITH_EXCEPT_START = 49,
    GET_AITER = 50,
    GET_ANEXT = 51,
    BEFORE_ASYNC_WITH = 52,
    END_ASYNC_FOR = 54,
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
    GET_YIELD_FROM_ITER = 69,
    PRINT_EXPR = 70,
    LOAD_BUILD_CLASS = 71,
    YIELD_FROM = 72,
    GET_AWAITABLE = 73,
    LOAD_ASSERTION_ERROR = 74,
    INPLACE_LSHIFT = 75,
    INPLACE_RSHIFT = 76,
    INPLACE_AND = 77,
    INPLACE_XOR = 78,
    INPLACE_OR = 79,
    LIST_TO_TUPLE = 82,
    RETURN_VALUE = 83,
    IMPORT_STAR = 84,
    SETUP_ANNOTATIONS = 85,
    YIELD_VALUE = 86,
    POP_BLOCK = 87,
    POP_EXCEPT = 89,

    // Opcodes with arguments
    STORE_NAME = 90,
    DELETE_NAME = 91,
    UNPACK_SEUQNECE = 92,
    FOR_ITER = 93,
    UNPACK_EX = 94,
    STORE_ATTR = 95,
    DELETE_ATTR = 96,
    STORE_GLOBAL = 97,
    DELETE_GLOBAL = 98,
    LOAD_CONST = 100,
    LOAD_NAME = 101,
    BUILD_TUPLE = 102,
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
    IS_OP = 117,
    CONTAINS_OP = 118,
    JUMP_IF_NOT_EXC_MATCH = 121,
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
    CALL_FUNCTION_KW = 141,
    CALL_FUNCTION_EX = 142,
    SETUP_WITH = 143,
    LIST_APPEND = 145,
    SET_ADD = 146,
    MAP_ADD = 147,
    // Python 3
    // LOAD_CLASSDEREF = 148,
    // SETUP_ASYNC_WITH = 154,
    // FORMAT_VALUE = 155,
    // BUILD_CONST_KEY_MAP = 156,
    // BUILD_STRING = 157,
    // LOAD_METHOD = 160,
    // CALL_METHOD = 161,
    // LIST_EXTEND = 162,
    // SET_UPDATE = 163,
    // DICT_MERGE = 164,
}

impl Opcode {
    /// Whether or not this opcode has an argument
    pub fn has_arg(&self) -> bool {
        *self as u8 >= 90
    }

    /// Whether or not this opcode has an extended argument
    pub fn has_extended_arg(&self) -> bool {
        *self as u8 >= 144
    }

    /// Whether or not this opcode has a constant parameter
    pub fn has_const(&self) -> bool {
        *self == Opcode::LOAD_CONST
    }

    /// Whether or not this opcode is a boolean operation
    pub fn has_comp(&self) -> bool {
        matches!(self, Opcode::COMPARE_OP)
    }

    /// Whether or not this opcode has a relative jump target
    pub fn is_relative_jump(&self) -> bool {
        matches!(
            self,
            Opcode::FOR_ITER
                //| Opcode::FORWARD
                | Opcode::SETUP_FINALLY
                | Opcode::SETUP_WITH
                //| Opcode::SETUP_ASYNC_WITH
        )
    }

    /// Whether or not this opcode has an absolute jump target
    pub fn is_absolute_jump(&self) -> bool {
        matches!(
            self,
            Opcode::JUMP_IF_FALSE_OR_POP
                | Opcode::JUMP_IF_TRUE_OR_POP
                | Opcode::JUMP_ABSOLUTE
                | Opcode::POP_JUMP_IF_FALSE
                | Opcode::POP_JUMP_IF_TRUE
                | Opcode::JUMP_IF_NOT_EXC_MATCH
        )
    }

    /// Whether or not this opcode accesses an attribute by name
    pub fn has_name(&self) -> bool {
        matches!(
            self,
            Opcode::STORE_NAME
                | Opcode::DELETE_NAME
                | Opcode::STORE_ATTR
                | Opcode::DELETE_ATTR
                | Opcode::STORE_GLOBAL
                | Opcode::DELETE_GLOBAL
                | Opcode::LOAD_NAME
                | Opcode::LOAD_ATTR
                | Opcode::IMPORT_NAME
                | Opcode::IMPORT_FROM
        )
    }

    /// Whether or not this opcode accesses a local variable
    pub fn has_local(&self) -> bool {
        matches!(
            self,
            Opcode::LOAD_FAST | Opcode::STORE_FAST | Opcode::DELETE_FAST //| Opcode::LOAD_METHOD
        )
    }

    /// Whether or not this opcode accesses a free variable
    pub fn has_free(&self) -> bool {
        matches!(
            self,
            Opcode::LOAD_CLOSURE
                | Opcode::LOAD_DEREF
                | Opcode::STORE_DEREF
        )
    }
}
