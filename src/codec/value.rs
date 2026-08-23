use std::ffi::{CStr, CString, c_void};

use crate::codec::AcceptedTypes;

#[repr(u8)]
pub enum Value<'a, T: AcceptedTypes> {
    Null,
    Unit,

    Bool(bool),
    Char(char),

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Isize(isize),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),

    Float32(f32),
    Float64(f64),

    Str(&'a str),
    String(String),

    CStr(&'a CStr),
    CString(CString),

    Bytes(&'a [u8]),
    ByteVec(Vec<u8>),

    Ptr(*const c_void),
    MutPtr(*mut c_void),
    Handle(u64),
    FnPtr(*const c_void),

    Ref(&'a Value<'a, T>),
    MutRef(&'a mut Value<'a, T>),

    Slice(&'a [Value<'a, T>]),
    Array(Box<[Value<'a, T>]>),
    Tuple(Box<[Value<'a, T>]>),

    Vec(Vec<Value<'a, T>>),
    Set(Vec<Value<'a, T>>),

    Record(Vec<(String, Value<'a, T>)>),
    Map(Vec<(Value<'a, T>, Value<'a, T>)>),

    Option(Option<Box<Value<'a, T>>>),
    Result(Result<Box<Value<'a, T>>, Box<Value<'a, T>>>),

    Generic(T),
}

impl<'a, T: AcceptedTypes> Value<'a, T> {
    #[inline]
    pub fn tag(&self) -> u8 {
        unsafe { *(self as *const _ as *const u8) }
    }
}

pub struct Null();

pub struct Handle(pub u64);

pub struct FnPtr(pub *const c_void);

pub struct Array<'a, T: AcceptedTypes>(pub Box<[Value<'a, T>]>);

pub struct Tuple<'a, T: AcceptedTypes>(pub Box<[Value<'a, T>]>);
