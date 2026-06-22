use crate::parser::Block;

enum Sign {
    Unsign,
    Sign,
}

enum IntegerSize {
    u8(i8),
    u16(i16),
    u32(i32),
    u64(i64),
}

struct Integer {
    integer_size: IntegerSize,
    sign: Sign,
}

enum FloatSize {
    f32(f32),
    f64(f64),
}

struct Float {
    float_size: FloatSize,
    sign: Sign,
}

enum Variables {
    Integer(Integer),
    Float(Float)
}

enum Types {
    Functions(fn()),
    Variable(Variables),
}
