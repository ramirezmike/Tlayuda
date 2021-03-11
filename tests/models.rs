use tob::*;

#[derive(tob, Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub ssn: String,
}

#[derive(tob, Debug)]
pub struct Teacher {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub has_class: bool,
}

#[derive(Debug)]
pub struct NonTobUsingStruct
{
    pub id: u32,
}

#[derive(tob, Debug)]
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
}
