//! [![github]](https://github.com/ramirezmike/Tlayuda)&ensp;[![crates-io]](https://crates.io/crates/tlayuda)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Tlayuda is a derive macro for structs that adds a static method which generates 
//! instances of the struct with minimal configuration. The goal is to provide an 
//! easy way to generate dynamic test data while only needing to setup the fields 
//! of an object that are relevant to a given unit test.
//!
//! # Example Usage
//! ```
//! #   pub struct StudentsPartitionedByGrade {
//! #       a_students: Vec::<Student>,
//! #       b_students: Vec::<Student>,
//! #       c_students: Vec::<Student>,
//! #       d_students: Vec::<Student>,
//! #       f_students: Vec::<Student>,
//! #   }
//! pub struct Student {      
//!     id: u64,
//!     first_name: String,
//!     last_name: String,
//!     telephone: String,
//!     date_of_birth: String,
//!     final_grade: u32,
//! }
//!
//! fn group_students_by_grade(mut students: Vec<Student>) -> StudentsPartitionedByGrade {
//!     let result = StudentsPartitionedByGrade {
//!         a_students: Vec::new(),
//!         b_students: Vec::new(),
//!         c_students: Vec::new(),
//!         d_students: Vec::new(),
//!         f_students: Vec::new(),
//!     };
//!
//!     students.drain(..).fold(result, |mut acc, student| {
//!         match student.final_grade {
//!             90..=100 => acc.a_students.push(student),
//!             80..=89 => acc.b_students.push(student),
//!             70..=79 => acc.c_students.push(student),
//!             60..=69 => acc.d_students.push(student),
//!             0..=50 => acc.f_students.push(student),
//!             _ => (),
//!         }
//!
//!         acc
//!     })
//! }
//! ```
//!   Given the above code, a unit test covering all aspects of `group_students_by_grade` 
//!   would require manually building a number of `Student` instances along with setting fields 
//!   that aren't even used in the function being tested. Tlayuda assists with populating the 
//!   fake data and exposing functions to set just the fields you care about.
//!
//! <br>
//!   Simply place the derive macro above a struct: 
//!
//! ```
//! use crate::tlayuda::*;
//!
//! #[derive(Tlayuda)]
//! pub struct Student { /* --snip-- */ }
//! ```
//! 
//! <br>
//!  With the macro in place, a unit test like the following can be written without needing
//!  to create a struct-specific builder. It's all created automatically.
//!
//! ```
//! #[test]
//! fn test_group_students_by_grade() {
//!     // sets up a vec of students with 10 students per number grade
//!     let students = Student::tlayuda()                  // create tlayuda test builder
//!         .set_final_grade(|index| (index % 101) as u32) // returns a 0-100 grade based on index
//!         .build_vec(200);                               // creates a vec of 200 students
//!
//!     // call function we're testing
//!     let result = group_students_by_grade(students);
//!
//!     // verifies expected # of students per group
//!     assert_eq!(20, result.a_students.len());
//!     assert_eq!(20, result.b_students.len());
//!     assert_eq!(20, result.c_students.len());
//!     assert_eq!(20, result.d_students.len());
//!     assert_eq!(102, result.f_students.len());
//!
//!     // verifies every group has the correct grade range
//!     result.a_students.iter().for_each(|s| assert!(s.final_grade >= 90 && s.final_grade <= 100));
//!     result.b_students.iter().for_each(|s| assert!(s.final_grade >= 80 && s.final_grade < 90));
//!     result.c_students.iter().for_each(|s| assert!(s.final_grade >= 70 && s.final_grade < 80));
//!     result.d_students.iter().for_each(|s| assert!(s.final_grade >= 60 && s.final_grade < 70));
//!     result.f_students.iter().for_each(|s| assert!(s.final_grade <= 50));
//! }
//! ```
//! # How To Use
//! Add the Tlayuda derive macro above a struct.
//!
//! ```
//! # use crate::tlayuda::*;
//! #[derive(Tlayuda)]
//! pub struct Person {
//!     id: u32,
//!     first_name: String,
//!     last_name: String,
//!     is_active: bool
//! }
//! ```
//!
//!  This will generate a builder struct, `Tlayuda{}Builder`, using the source struct's name. 
//!  A static method will be added on the source struct, `tlayuda`, that returns an instance 
//!  of the builder struct. Calling `build()` on the builder object generates an instance of 
//!  the source struct using "dynamic defaults" for each field based on the field's type 
//!  and its index (an incrementing ID stored by the builder).
//!
//! ```
//! # use crate::tlayuda::*;
//! #   #[derive(Tlayuda)]
//! #   pub struct Person {
//! #       id: u32,
//! #       first_name: String,
//! #       last_name: String,
//! #       is_active: bool
//! #   }
//! # #[test]
//! # fn test() {
//! /* inside a test */
//! let mut builder = Person::tlayuda();
//! let person = builder.build();
//!
//! assert_eq!(0, person.id);
//! assert_eq!("first_name0", person.first_name);
//! assert_eq!("last_name0", person.last_name);
//! assert_eq!(false, person.is_active);
//!
//! // builder increments the internal index
//! // with each call to build
//! let person = builder.build();
//!
//! assert_eq!(1, person.id);
//! assert_eq!("first_name1", person.first_name);
//! assert_eq!("last_name1", person.last_name);
//! assert_eq!(false, person.is_active);
//! # }
//! ```
//!
//! The builder will also have a `set_` prefixed method for each field in the struct that 
//! takes a closure of the form `FnMut(usize) -> Type`. The `usize` parameter is the "index" 
//! of the built object. The return type of the closure should match the type of the field. 
//! This can be used to setup customized field values without needing to setup entire structs 
//! with values that are irrelevant to the current test.
//!
//! ```
//! # use crate::tlayuda::*;
//! #   #[derive(Tlayuda)]
//! #   pub struct Person {
//! #       id: u32,
//! #       first_name: String,
//! #       last_name: String,
//! #       is_active: bool
//! #   }
//! # #[test]
//! # fn test() {
//! /* inside a test */
//! let mut builder = Person::tlayuda()
//!     .set_first_name(|i| {
//!         if i == 1 {
//!             "Michael".to_string()
//!         } else {
//!             format!("first_name{}", i)
//!         }
//!     });
//!  
//! let person = builder.build();
//! assert_eq!("first_name0", person.first_name);
//!  
//! let person = builder.build();
//! assert_eq!("Michael", person.first_name);
//! # }
//! ```
//! 
//! The builder can also generate a `Vec::<_>` of the struct with a call to `build_vec`. 
//! This internally uses the builder's current settings to generate data while 
//! incrementing the index after each build.
//!
//! ```
//! #   use crate::tlayuda::*;
//! #   #[derive(Tlayuda)]
//! #   pub struct Person {
//! #       id: u32,
//! #       first_name: String,
//! #       last_name: String,
//! #       is_active: bool
//! #   }
//! # #[test]
//! # fn test() {
//! // builds 1000 Person objects and verifies
//! // each object has a unique first_name value
//! Person::tlayuda()
//!     .set_first_name(|i| i.to_string())
//!     .build_vec(1000)
//!     .iter()
//!     .enumerate()
//!     .for_each(|(i, x)| {
//!         assert_eq!(i.to_string(), x.first_name);
//!     });
//! # }
//! ```
//!
//! You can also change the starting index of the builder with a call to 
//! `with_index(index: usize)`. Additionally, every call to the builder object 
//! (excluding calls to `build` or `build_vec`) will return the builder to allow chaining. 
//!
//! ```
//! #   use crate::tlayuda::*;
//! #   #[derive(Tlayuda)]
//! #   pub struct Person {
//! #       id: u32,
//! #       first_name: String,
//! #       last_name: String,
//! #       is_active: bool
//! #   }
//! # #[test]
//! # fn test() {
//! /* inside a test */
//! Person::tlayuda()
//!     .set_first_name(|i| match (i % 3, i % 5) {
//!         (0, 0) => "FizzBuzz".into(),
//!         (0, _) => "Fizz".into(),
//!         (_, 0) => "Buzz".into(),
//!         _ => i.to_string(),
//!     })
//!     .with_index(1)
//!     .build_vec(100)
//! # }
//! ```
//!
//! Tlayuda will also automatically attempt to recursively build fields if they're 
//! not one of the known supported types. That is, if `struct A` has a field that 
//! is `struct B` which also has the Tlayuda derive macro, the `struct A` builder 
//! will automatically call `struct B`'s builder. Note: this will cause compile 
//! errors if the inner struct has unsupported fields or doesn't use the Tlayuda macro.
//!
//! ```
//! # use crate::tlayuda::*;
//! #[derive(Tlayuda)]
//! pub struct StructA {
//!     pub some_field: u32,
//!     pub another_struct: StructB,
//! }
//!
//! #[derive(Tlayuda)]
//! pub struct StructB {
//!     pub field_on_b: String,
//! }
//!
//! # #[test]
//! # fn test() {
//! /* inside a test */
//! let some_A = StructA::tlayuda().build();
//! assert_eq!("field_on_b0", some_A.another_struct.field_on_struct_b);
//! # }
//! ```
//!
//! # Supported Types
//! 
//!
//! Currently Tlayuda supports structs that are **solely** composed of the following
//!
//! * numeric primitives (i8-i128, u8-u128, f32, f64, isize, usize)
//! * bools
//! * char
//! * String, OsString
//! * Vecs
//! * Arrays with numeric primitives
//! * structs composed **solely** from the above types (and that are using the Tlayuda macro)
//!
//! Types with full paths will have their paths ignored and behave as whatever the 
//! last segment is. I.e., "std::ffi::OsString" will be treated as "OsString."
//!
//! While the goal is to support as many types as possible, it's currently likely 
//! to run into unsupported types. Adding a `tlayuda_ignore` attribute above an 
//! unsupported field will mark that field to be skipped. Instead, the `.tlayuda()` 
//! function will be modified to take a parameter of that type which will be cloned 
//! to populate that field during the build process.
//!
//! ```
//!     use crate::tlayuda::*;
//!
//!     #[derive(Tlayuda)]
//!     pub struct StructA {
//!         pub some_field: u32,
//!         pub some_other_field: bool,
//!         #[tlayuda_ignore] // add attribute above unsupported types
//!         pub some_unsupported_type: Vec::<u32>,
//!     }
//!
//! # #[test]
//! # fn test() {
//!     /* inside a test */
//!
//!     // construct a value for the unsupported type
//!     let some_vec: Vec::<u32> = vec![1, 2, 3]; 
//!
//!     // ignored field now required as a parameter instead of being handled by tlayuda
//!     let mut builder = structA::tlayuda(some_vec); 
//!     let some_1 = builder.build(); 
//!
//!     // value gets populated with value passed into tlayuda()
//!     assert_eq!(100, some_1.some_unsupported_type[0]);
//!
//!     let some_2 = builder.build(); 
//!     assert_eq!(100, some_2.some_unsupported_type[0]); // value is cloned across builds
//! # }
//! ```
//!
//! # Running outside of Tests
//! By default, Tlayuda only works while executing tests; the macro outputs code
//! using a cfg[(test)] attribute so it only affects tests. While the construction 
//! of objects should remain consistent across versions of Tlayuda, the intent and
//! design of the generated code is intended for testing purposes. If you have a
//! use-case for using Tlayuda outside of tests, you can do so by enabling the
//! "allow_outside_tests" feature.
//!
//!
//! # Tlayuda Anti-patterns
//! A common anti-pattern to avoid when using Tlayuda is writing asserts based on
//! the default outputs of Tlayuda when a custom value would be a better indicator
//! that your code is working as expected.
//!
//! ```
//! # use crate::tlayuda::*;
//! #   #[derive(Tlayuda)]
//! #   pub struct Person {
//! #       id: u32,
//! #       first_name: String,
//! #       last_name: String,
//! #       is_active: bool
//! #   }
//! # // implementation isn't important, this is just a stub for demonstration
//! # #[test]
//! # fn get_by_id(_id: usize) -> Person { Person::tlayuda().build() }
//! # #[test]
//! # fn test() {
//! /* inside a test */
//! let person = get_by_id(0); // some function using generated data
//! assert_eq!("first_name0", person.first_name); // BAD
//! # }
//! ```
//!
//! While this works, this is a bad pattern because it's possible the assert
//! was written to **match** the output rather than **predict** the output.
//! An assert like this can hide a bug in the function you're testing and seeing
//! the default generated value in the assert is an indication that the test
//! was written to match rather than predict the results.
//! 
//! Instead, use the `set_` functions to set the expected return value. 
//!
//! ```
//! # use crate::tlayuda::*;
//! #   #[derive(Tlayuda)]
//! #   pub struct Person {
//! #       id: u32,
//! #       first_name: String,
//! #       last_name: String,
//! #       is_active: bool
//! #   }
//! # // implementation isn't important, this is just a stub for demonstration
//! # #[test]
//! # fn get_by_id(_id: usize) -> Person { Person::tlayuda().build() }
//! # #[test]
//! # fn test() {
//! // create a custom builder and use this to generate data
//! let builder = Person::tlayuda()
//!                     .set_first_name(|i| {
//!                         if i == 0 {
//!                             "Michael".to_string()
//!                         } else {
//!                             format!("first_name{}", i)
//!                         }
//!                     });
//! 
//! /* inside a test */
//! let person = get_by_id(0); // some function using generated data
//! assert_eq!("Michael", person.first_name); // GOOD
//! # }
//! ```
//!
//! # Example Output
//!
//! This shows roughly what Tlayuda actually outputs when deriving the given struct
//!
//! 
//! ```
//! use crate::tlayuda::*;
//!
//! // given this struct
//! #[derive(Tlayuda)]
//! struct Person {
//!     id: u32,
//!     first_name: String,
//!     last_name: String,
//!     is_active: bool,
//!     #[tlayuda_ignore]
//!     friends: Vec::<u32>,
//! }
//!
//! // Tlayuda generates the following
//!
//! pub struct TlayudaPersonBuilder {
//!     index: usize,
//!     id: Box<dyn FnMut(usize) -> u32>,
//!     first_name: Box<dyn FnMut(usize) -> String>,
//!     last_name: Box<dyn FnMut(usize) -> String>,
//!     is_active: Box<dyn FnMut(usize) -> bool>,
//!     friends: Vec::<u32>,
//! }
//! impl TlayudaPersonBuilder {
//!     pub fn new(friends: Vec::<u32>) -> TlayudaPersonBuilder {
//!         TlayudaPersonBuilder {
//!             index: 0,
//!             id: Box::new( |i| i as u32 ),
//!             first_name: Box::new( |i| format!("{}{}", "first_name", i).into() ),
//!             last_name: Box::new( |i| format!("{}{}", "last_name", i).into() ),
//!             is_active: Box::new( |i| false ),
//!             friends: friends,
//!         }
//!     }
//!
//!     pub fn set_id<F: 'static>(mut self, f: F) -> Self where
//!         F: Fn(usize) -> u32 {
//!             self.id = Box::new(f);
//!             self
//!     }
//!
//!     pub fn set_first_name<F: 'static>(mut self, f: F) -> Self where
//!         F: Fn(usize) -> String {
//!             self.first_name = Box::new(f);
//!             self
//!     }
//!
//!     pub fn set_last_name<F: 'static>(mut self, f: F) -> Self where
//!         F: Fn(usize) -> String {
//!             self.last_name = Box::new(f);
//!             self
//!     }
//!
//!     pub fn set_is_active<F: 'static>(mut self, f: F) -> Self where
//!         F: Fn(usize) -> bool {
//!             self.is_active = Box::new(f);
//!             self
//!     }
//!
//!     pub fn with_index(mut self, index: usize) -> Self {
//!         self.index = index;
//!         self
//!     }
//!
//!     fn take_index(&mut self) -> usize {
//!         self.index = self.index + 1;
//!         self.index - 1
//!     }
//!
//!     pub fn build(&mut self) -> Person {
//!         let i = self.take_index();
//!         Person {
//!             friends: self.friends.clone(),
//!             id: self.id.as_mut()(i),
//!             first_name: self.first_name.as_mut()(i),
//!             last_name: self.last_name.as_mut()(i),
//!             is_active: self.is_active.as_mut()(i),
//!         }
//!     }
//!
//!     pub fn build_vec(&mut self, count: usize) -> Vec::<Person> {
//!         std::iter::repeat_with(|| self.build()).take(count).collect()
//!     }
//! }
//!
//! impl Person {
//!     pub fn tlayuda(friends: Vec::<u32>) -> TlayudaPersonBuilder {
//!         TlayudaPersonBuilder::new(friends)
//!     }
//! }
//! ```
//!

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct, Meta, Type};


/// A derive macro that generates a test data builder for a struct
#[proc_macro_derive(Tlayuda, attributes(tlayuda_ignore))]
pub fn entry_point(input: TokenStream) -> TokenStream {
    let source_struct = parse_macro_input!(input as ItemStruct);
    let source_struct_name = source_struct.ident.clone();
    let fields = get_fields(source_struct);
    let inner_builder_name = quote::format_ident!("Tlayuda{}Builder", source_struct_name);

    let OutputTokenPartials {
        field_declarations,
        field_builder_intializers,
        field_setter_functions,
    } = generate_output_tokens(&fields);

    let builder_parameters = fields
        .iter()
        .filter(|f| f.is_ignored)
        .map(|f| {
            let identifier = &f.identifier;
            let field_type = &f.field_type;

            quote! { #identifier: #field_type }
        })
        .collect::<Vec<_>>();

    let (ignored_fields, fields): (Vec<_>, Vec<_>) = fields.iter().partition(|f| f.is_ignored);

    // Ignored fields will be manually populated by the user with a clonable
    // instance passed into the initial .tlayuda() call. The following
    // is intended to create those parameters.
    let inner_builder_constructor_parameters = ignored_fields.iter()
                                                             .map(|f| {
                                                                 let i = &f.identifier;
                                                                 quote! { #i }
                                                             });
    let ignored_fields = ignored_fields.iter()
                                       .map(|f| {
                                           let inner_identifier = quote::format_ident!("inner_{}", f.identifier);
                                           let identifier = &f.identifier;
                                           quote! { #identifier: self.#inner_identifier.clone(), }
                                       });

    let fields = fields.iter()
                       .map(|f| {
                           let inner_identifier = quote::format_ident!("inner_{}", f.identifier);
                           let identifier = &f.identifier;
                           quote! { #identifier: self.#inner_identifier.as_mut()(i), }
                       });

    let output = quote! {
        #[cfg(any(test, feature="allow_outside_tests"))]
        pub struct #inner_builder_name {
            index: usize,
            #(#field_declarations),*
        }

        #[cfg(any(test, feature="allow_outside_tests"))]
        impl #inner_builder_name {
            pub fn new(#(#builder_parameters),*) -> #inner_builder_name {
                #inner_builder_name {
                    index: 0,
                    #(#field_builder_intializers),*
                }
            }

            #(#field_setter_functions)*

            pub fn with_index(mut self, index: usize) -> Self {
                self.index = index;
                self
            }

            fn take_index(&mut self) -> usize {
                self.index = self.index + 1;
                self.index - 1
            }

            pub fn build(&mut self) -> #source_struct_name {
                let i = self.take_index();
                #source_struct_name {
                    #(#ignored_fields)*
                    #(#fields)*
                }
            }

            pub fn build_vec(&mut self, count: usize) -> Vec::<#source_struct_name> {
                std::iter::repeat_with(|| self.build()).take(count).collect()
            }
        }

        #[cfg(any(test, feature="allow_outside_tests"))]
        impl #source_struct_name {
            pub fn tlayuda(#(#builder_parameters),*) -> #inner_builder_name {
                #inner_builder_name::new(#(#inner_builder_constructor_parameters),* )
            }
        }
    };

    TokenStream::from(output)
}

#[derive(Debug)]
struct FieldInfo {
    identifier: proc_macro2::Ident,
    field_type: syn::Type,
    is_ignored: bool,
}

fn get_fields(item_struct: ItemStruct) -> Vec<FieldInfo> {
    item_struct
        .fields
        .iter()
        .filter(|x| x.ident.is_some())
        .map(|x| FieldInfo {
            identifier: x.ident.as_ref().unwrap().clone(),
            field_type: x.ty.clone(),
            is_ignored: x.attrs.iter().any(|attribute| {
                if let Ok(meta) = attribute.parse_meta() {
                    match meta {
                        Meta::Path(path) => path.is_ident("tlayuda_ignore".into()),
                        _ => false,
                    }
                } else {
                    false
                }
            }),
        })
        .collect()
}

struct OutputTokenPartials {
    field_setter_functions: Vec<proc_macro2::TokenStream>,
    field_builder_intializers: Vec<proc_macro2::TokenStream>,
    field_declarations: Vec<proc_macro2::TokenStream>,
}

fn generate_output_tokens(fields: &Vec<FieldInfo>) -> OutputTokenPartials {
    let field_setter_functions = fields
        .iter()
        .filter(|f| !f.is_ignored)
        .map(|field| {
            let set_func_name = quote::format_ident!("set_{}", field.identifier);
            let identifier = quote::format_ident!("inner_{}", field.identifier);
            let field_type = &field.field_type;

            quote! {
                pub fn #set_func_name<F: 'static>(mut self, f: F) -> Self where
                    F: Fn(usize) -> #field_type {
                        self.#identifier = Box::new(f);
                        self
                }
            }
        })
        .collect();

    let field_builder_intializers = fields
        .iter()
        .map(|field| {
            let inner_identifier = quote::format_ident!("inner_{}", field.identifier);
            let identifier = &field.identifier;

            if field.is_ignored {
                let value = &field.identifier;
                quote! { #inner_identifier: #value }
            } else {
                let field_type = parse_field_type(&field.field_type);

                let f =
                match field_type {
                    FieldType::Basic(field_type, full_field_type) => {
                        match field_type.to_string().as_str() {
                            "String" => quote! { |i| format!("{}{}", stringify!(#identifier), i).into() },
                            "OsString" => quote! { |i| format!("{}{}", stringify!(#identifier), i).into() },
                            "char" => quote! { |i| std::char::from_digit(i as u32, 10).unwrap_or('a') },
                            "bool" => quote! { |i| false },
                            "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "i64" | "i128" | "isize"
                            | "u64" | "u128" | "usize" | "f32" | "f64" => {
                                quote! { |i| i as #full_field_type }
                            },
                            "Vec" => quote! { |i| Vec::new() },
                            _ => {
                                // attempt to call a builder that may be on this type
                                // this will end up causing a compile error if the type doesn't have
                                // the #[derive(Tlayuda)] macro.
                                // TODO: Need to figure out a way to communicate this better in the compiler
                                quote! { |i| #full_field_type::tlayuda().with_index(i).build() }
                            }
                        }
                    },
                    FieldType::Array(field_type, full_field_type, length) => {
                        match field_type.to_string().as_str() {
                            "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "i64" | "i128" | "isize"
                            | "u64" | "u128" | "usize" | "f32" | "f64" => {
                                quote! { |i| [i as #full_field_type; #length] }
                            },
                            _ => panic!("Type {:?} not yet supported for arrays", field_type)
                        }
                    }
                };

                quote! { #inner_identifier: Box::new(#f) }
            }
        })
        .collect();

    let field_declarations = fields
        .iter()
        .map(
            |FieldInfo {
                 identifier,
                 field_type,
                 is_ignored,
             }| {
                let identifier = quote::format_ident!("inner_{}", identifier);
                if *is_ignored {
                    quote! { #identifier: #field_type }
                } else {
                    quote! {
                        #identifier: Box<dyn FnMut(usize) -> #field_type>
                    }
                }
            },
        )
        .collect();

    OutputTokenPartials {
        field_declarations,
        field_builder_intializers,
        field_setter_functions,
    }
}

enum FieldType {
    Basic(syn::Ident, proc_macro2::TokenStream),
    Array(syn::Ident, proc_macro2::TokenStream, usize),
}

fn parse_field_type(field_type: &syn::Type) -> FieldType {
    match field_type {
        Type::Path(type_path) => match type_path.path.get_ident() {
            Some(ident) => FieldType::Basic(ident.clone(), ident.into_token_stream()),
            None => (
                FieldType::Basic(type_path.path.segments.last().unwrap().ident.clone(),
                                 type_path.into_token_stream())
            ),
        },
        Type::Array(type_array) => {
            match parse_field_type(&type_array.elem) {
                FieldType::Basic(i, ts) => {
                    match &type_array.len {
                        syn::Expr::Lit(expr) => {
                            match &expr.lit {
                                syn::Lit::Int(number) => {
                                    match number.base10_parse::<usize>() {
                                        Ok(parsed_number) => FieldType::Array(i, ts, parsed_number),
                                        _ => panic!("Number literal in array was invalid: {:?}", number)
                                    }
                                },
                                _ => todo!("Array length literal {:?} not yet supported", expr)
                            }
                            
                        },
                        _ => todo!("Array length expression {:?} not yet supported", type_array.len)
                    }
                },
                _ => todo!("Nested arrays not yet supported")
            }
        },
        _ => todo!("Type {:?} not supported", field_type),
    }
}
