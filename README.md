# TOB (Test Object Builder)

A derive procedural macro for structs that adds a static method that can be used to easily generate objects for use in unit tests.

## Examples

```
#[derive(tob)] // 1.
pub struct Person {
    id: u32,
    first_name: String,
    last_name: String,
    is_active: bool
}

#[test]
fn example_one() {
    let person = Person::test_obj_builder() // 2. 
                     .build();              // 3.

    assert_eq!(0, person.id);                     // 4.
    assert_eq!("first_name0", person.first_name); // 5.
    assert_eq!("last_name0", person.last_name);
    assert_eq!(false, person.is_active);          // 6.
}

```

1. add macro above struct
2. "test_obj_builder" static method gets automatically added which returns a customized builder object with "set_" prefixed functions for each field on the struct.
3. calling "build()" generates an instance and increments the internal index 
4. default behavior of numeric-types is to return the current index
5. default behavior of Strings is to return the name of the field post-fixed with the current index
6. default behavior of bools return is to false
