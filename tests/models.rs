use std::ffi::OsString;
use tlayuda::Tlayuda;

#[derive(Tlayuda, Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub ssn: String,
}

#[derive(Tlayuda, Debug)]
pub struct Teacher {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub has_class: bool,
}

#[derive(Debug)]
pub struct NonTlayudaUsingStruct {
    pub id: u32,
}

#[derive(Tlayuda, Debug)]
pub struct IgnoreTester {
    pub type_string: String,
    #[tlayuda_ignore]
    pub type_bool: bool,
    pub type_i8: i8,
    #[tlayuda_ignore]
    pub type_vec_u32: Vec<u32>,
}

#[derive(Tlayuda, Debug)]
pub struct TypeTester {
    pub type_string: String,
    pub type_bool: bool,
    pub type_i8: i8,
    pub type_i16: i16,
    pub type_i32: i32,
    pub type_u8: u8,
    pub type_u16: u16,
    pub type_u32: u32,
    pub type_i64: i64,
    pub type_i128: i128,
    pub type_isize: isize,
    pub type_u64: u64,
    pub type_u128: u128,
    pub type_usize: usize,
    pub type_f32: f32,
    pub type_f64: f64,
    pub type_char: char,
    pub type_person: Person,
    pub type_osstring: OsString,
    pub type_full_path: std::ffi::OsString,
    pub type_vec_u32: Vec::<u32>,
    pub type_array_i8: [i8; 3],
    pub type_array_i16: [i16; 3],
    pub type_array_i32: [i32; 3],
    pub type_array_u8: [u8; 3],
    pub type_array_u16: [u16; 3],
    pub type_array_u32: [u32; 3],
    pub type_array_i64: [i64; 3],
    pub type_array_i128: [i128; 3],
    pub type_array_isize: [isize; 3],
    pub type_array_u64: [u64; 3],
    pub type_array_u128: [u128; 3],
    pub type_array_usize: [usize; 3],
    pub type_array_f32: [f32; 3],
    pub type_array_f64: [f64; 3],
}
