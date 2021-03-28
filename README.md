[![github]](https://github.com/ramirezmike/Tlayuda)&ensp;[![crates-io]](https://crates.io/crates/tlayuda)&ensp;[![docs-rs]](https://docs.rs/tlayuda/)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

# Tlayuda - a test object builder

A derive procedural macro for structs that adds a static method that generates instances of the struct with minimal configuration. The goal is to provide an easy way to generate dynamic test data while only needing to setup the fields of an object that are relevant to a given unit test.

## Usage Example
```
#[derive(Tlayuda)]
pub struct Student {
    id: u64,
    first_name: String,
    last_name: String,
    telephone: String,
    date_of_birth: String,
    final_grade: u32,
}

fn group_students_by_grade(mut students: Vec<Student>) -> StudentsPartitionedByGrade {
    let result = StudentsPartitionedByGrade {
        a_students: Vec::new(),
        b_students: Vec::new(),
        c_students: Vec::new(),
        d_students: Vec::new(),
        f_students: Vec::new(),
    };

    students.drain(..).fold(result, |mut acc, student| {
        match student.final_grade {
            90..=100 => acc.a_students.push(student),
            80..=89 => acc.b_students.push(student),
            70..=79 => acc.c_students.push(student),
            60..=69 => acc.d_students.push(student),
            0..=50 => acc.f_students.push(student),
            _ => (),
        }

        acc
    })
}
```

Given the above code, a unit test covering all aspects of `group_students_by_grade` would require manually building a number of Student instances along with setting fields that aren't even used in the function being tested. Tlayuda assists with populating the fake data and exposing functions to set just the fields you care about.

```
#[test]
fn test_group_students_by_grade() {
    // sets up a vec of students with 10 students per number grade
    let students = Student::tlayuda()                  // create tlayuda test builder
        .set_final_grade(|index| (index % 101) as u32) // returns a 0-100 grade based on index
        .build_vec(200);                               // creates a vec of 200 students

    // call function we're testing
    let result = group_students_by_grade(students);

    // verifies expected # of students per group
    assert_eq!(20, result.a_students.len());
    assert_eq!(20, result.b_students.len());
    assert_eq!(20, result.c_students.len());
    assert_eq!(20, result.d_students.len());
    assert_eq!(102, result.f_students.len());

    // verifies every group has the correct grade range
    result.a_students.iter().for_each(|s| assert!(s.final_grade >= 90 && s.final_grade <= 100));
    result.b_students.iter().for_each(|s| assert!(s.final_grade >= 80 && s.final_grade < 90));
    result.c_students.iter().for_each(|s| assert!(s.final_grade >= 70 && s.final_grade < 80));
    result.d_students.iter().for_each(|s| assert!(s.final_grade >= 60 && s.final_grade < 70));
    result.f_students.iter().for_each(|s| assert!(s.final_grade <= 50));
}
```
[Full code example here](https://github.com/ramirezmike/Tlayuda/blob/main/tests/usage_example.rs)


## How To Use

Add the `Tlayuda` derive macro above a struct.

```
use tlayuda::*;

#[derive(Tlayuda)]
pub struct Person {
    id: u32,
    first_name: String,
    last_name: String,
    is_active: bool
}
```
This will generate a builder struct, `Tlayuda{}Builder`, using the source struct's name. A static method will be added on the source struct, `tlayuda`, that returns an instance of the builder struct. Calling `build()` on the builder object generates an instance of the source struct using "dynamic defaults" for each field based on the field's type and its index (an incrementing ID stored by the builder).

```
    let mut builder = Person::tlayuda();
    let person = builder.build();

    assert_eq!(0, person.id);
    assert_eq!("first_name0", person.first_name);
    assert_eq!("last_name0", person.last_name);
    assert_eq!(false, person.is_active);

    // builder increments the internal index 
    // with each call to build
    let person = builder.build();

    assert_eq!(1, person.id);
    assert_eq!("first_name1", person.first_name);
    assert_eq!("last_name1", person.last_name);
    assert_eq!(false, person.is_active);
```

The builder will also have a `set_` prefixed method for each field in the struct that takes a closure of the form `FnMut(usize) -> Type`. The `usize` parameter is the "index" of the built object. The return type of the closure should match the type of the field. This can be used to setup customized field values without needing to setup entire structs with values that are irrelevant to the current test.

```
    let mut builder = Person::tlayuda()
        .set_first_name(|i| {
            if i == 1 { 
                "Michael".to_string()
            } else {
                format!("first_name{}", i)
            }
        });
        
    let person = builder.build();
    assert_eq!("first_name0", person.first_name);

    let person = builder.build();
    assert_eq!("Michael", person.first_name);
```

The builder can also generate a `Vec::<_>` of the struct with a call to `build_vec`. This internally uses the builder's current settings to generate data while incrementing the index after each build.

```
    // builds 1000 Person objects and verifies
    // each object has a unique first_name value
    Person::tlayuda()
        .set_first_name(|i| i.to_string())
        .build_vec(1000)
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            assert_eq!(i.to_string(), x.first_name);
        });

```

You can also change the starting index of the builder with a call to `with_index(index: usize)`. Additionally, every call to the builder object (excluding calls to `build` or `build_vec`) will return the builder to allow chaining.

```
    Person::tlayuda()
        .set_first_name(|i| match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".into(),
            (0, _) => "Fizz".into(),
            (_, 0) => "Buzz".into(),
            _ => i.to_string(),
        })
        .with_index(1)
        .build_vec(100)
```

Tlayuda will also automatically attempt to recursively build fields if they're not one of the known supported types. That is, if `struct A` has a field that is `struct B` which also has the Tlayuda derive macro, the `struct A` builder will automatically call `struct B`'s builder. *Note: this will cause compile errors if the inner struct has unsupported fields or doesn't use the Tlayuda macro.*

```
    #[derive(Tlayuda)]
    pub struct StructA {
        pub some_field: u32,
        pub another_struct: StructB,
    }

    #[derive(Tlayuda)]
    pub struct StructB {
        pub field_on_b: String,
    }

    let some_A = StructA::tlayuda().build();
    assert_eq!("field_on_b0", some_A.another_struct.field_on_struct_b);
```

## Supported Types

Currently Tlayuda supports structs that are **solely** composed of the following

* numeric primitives (i8-i128, u8-u128, f32, f64, isize, usize)
* bools
* char
* String, OsString
* Vecs
* Arrays with numeric primitives
* structs composed **solely** from the above types (and that are using the Tlayuda macro)

Types with full paths will have their paths ignored and behave as whatever the last segment is. I.e., "std::ffi::OsString" will be treated as "OsString."

While the goal is to support as many types as possible, it's currently likely to run into unsupported types. Adding a `tlayuda_ignore` attribute above an unsupported field will mark that field to be skipped. Instead, the `.tlayuda()` function will be modified to take a parameter of that type which will be cloned to populate that field during the build process.

```
    #[derive(Tlayuda)]
    pub strut StructA {
        pub some_field: u32,
        pub some_other_field: bool,
        #[tlayuda_ignore] // add attribute above unsupported types
        pub some_unsupported_type: Vec::<u32>,
    }

    /* inside a test */

    let some_vec: Vec::<u32> = vec![1, 2, 3]; // construct a value for the unsupported type
    let mut builder = StructA::tlayuda(some_vec); // ignored field now required as a parameter instead of being handled by tlayuda
    let some_1 = builder.build(); 

    assert_eq!(100, some_1.some_unsupported_type[0]); // value gets populated with value passed into tlayuda()

    let some_2 = builder.build(); 
    assert_eq!(100, some_2.some_unsupported_type[0]); // value is cloned across builds
```

## Running outside of Tests
By default, Tlayuda only works while executing tests; the macro outputs code using a cfg[(test)] attribute so it only affects tests. While the construction of objects should remain consistent across versions of Tlayuda, the intent and design of the generated code is intended for testing purposes. If you have a use-case for using Tlayuda outside of tests, you can do so by enabling the "allow_outside_tests" feature.


## Current TODO list:
- [X] Add vec as a supported type
- [X] Fix failing Doc tests
- [ ] Add an "order" parameter to the tlayuda_ignore attribute to customize `tlayuda()` parameter order
- [ ] Add more type supports for arrays (including nested arrays)
- [ ] Add support for HashMaps 
- [ ] Add support for tuples consisting of current supported types
- [ ] Add matching access modifier (public/private) to avoid leaking private types
