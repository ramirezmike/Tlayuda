# Tlayuda - a test object builder

A derive procedural macro for structs that adds a static method that generates instances of the struct with minimal configuration. The goal is to provide an easy way to generate dynamic test data while only needing to setup the fields of an object that are relevant to a given unit test.

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

You can also change the starting index of the builder with a call to `with_index(index: usize)`. Additionally, every call to the builder object (excluding calls to `build` or `build_vec` return the builder to allow chaining.

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
* structs composed **solely** from the above types (and that are using the Tlayuda macro)

Types with full paths will have their paths ignored and behave as whatever the last segment is. I.e., "std::ffi::OsString" will be treated as "OsString."

While the goal is to support as many types as possible, it's currently likely to run into unsupported types. Adding a `tlayuda_ignore` attribute above an unsupported field will mark that field to be skipped. Instead, the `.tlayuda()` function will be modified to take a parameter of that type which will be cloned to populate that field during the build process.

```
    #[derive(Tlayuda)]
    pub structA {
        pub some_field: u32,
        pub some_other_field: bool,
        #[tlayuda_ignore] // add attribute above unsupported types
        pub some_unsupported_type: Vec::<u32>,
    }

    /* inside a test */

    let some_vec: Vec::<32> = vec![1, 2, 3]; // construct a value for the unsupported type
    let mut builder = structA.tlayuda(some_vec); // ignored field now required as a parameter instead of being handled by tlayuda
    let some_1 = builder.build(); 

    assert_eq!(100, some_1.some_unsupported_type[0]); // value gets populated with value passed into tlayuda()

    let some_2 = builder.build(); 
    assert_eq!(100, some_2.some_unsupported_type[0]); // value is cloned across builds
```

## Current TODO list:
- [ ] Add vec as a supported type
- [ ] Add an "order" parameter to the tlayuda_ignore attribute to customize `tlayuda()` parameter order
