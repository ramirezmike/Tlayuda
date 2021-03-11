mod models;
use std::sync::{Arc, Mutex};

#[test]
fn basic_build() {
    let mut builder = models::Person::test_obj_builder();
    let person_0 = builder.build();
    let person_1 = builder.build();
    let person_2 = builder.build();

    assert_eq!("first_name0", person_0.first_name);
    assert_eq!("last_name0", person_0.last_name);
    assert_eq!("ssn0", person_0.ssn);

    assert_eq!("first_name1", person_1.first_name);
    assert_eq!("last_name1", person_1.last_name);
    assert_eq!("ssn1", person_1.ssn);

    assert_eq!("first_name2", person_2.first_name);
    assert_eq!("last_name2", person_2.last_name);
    assert_eq!("ssn2", person_2.ssn);
}

#[test]
fn basic_build_vec() {
    models::Person::test_obj_builder()
        .build_vec(1000)
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            assert_eq!(format!("first_name{}", i), x.first_name);
            assert_eq!(format!("last_name{}", i), x.last_name);
            assert_eq!(format!("ssn{}", i), x.ssn);
        });
}

#[test]
fn verify_able_to_build_different_objects() {
    let person = models::Person::test_obj_builder().build();
    let teacher = models::Teacher::test_obj_builder().build();

    assert_eq!("first_name0", person.first_name);
    assert_eq!("last_name0", person.last_name);
    assert_eq!("ssn0", person.ssn);

    assert_eq!(0, teacher.id);
    assert_eq!("first_name0", teacher.first_name);
    assert_eq!("last_name0", teacher.last_name);
    assert_eq!(false, teacher.has_class);
}

#[test]
fn verify_basic_set_functions() {
    let expected_first_name: String = "Michael".into();
    let expected_last_name: String = "Ramirez".into();
    let person = models::Person::test_obj_builder()
        .set_first_name(move |_| expected_first_name.clone())
        .set_last_name(move |_| expected_last_name.clone())
        .build();
    assert_eq!("Michael", person.first_name);
    assert_eq!("Ramirez", person.last_name);
}

#[test]
fn verify_incrementing_index_in_set_functions_build() {
    let mut builder = models::Person::test_obj_builder()
        .set_first_name(|i| i.to_string())
        .set_last_name(|i| i.to_string());

    for i in 0..1000 {
        let person = builder.build();
        assert_eq!(i.to_string(), person.first_name);
        assert_eq!(i.to_string(), person.last_name);
    }
}

#[test]
fn verify_incrementing_index_in_set_functions_build_vec() {
    models::Person::test_obj_builder()
        .set_first_name(|i| i.to_string())
        .set_last_name(|i| i.to_string())
        .build_vec(1000)
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            assert_eq!(i.to_string(), x.first_name);
            assert_eq!(i.to_string(), x.last_name);
        });
}

#[test]
fn verify_set_function_build_vec_fizzbuzz() {
    let expected_results = vec![
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14",
        "FizzBuzz", "16", "17", "Fizz", "19", "Buzz", "Fizz", "22", "23", "Fizz", "Buzz", "26",
        "Fizz", "28", "29", "FizzBuzz", "31", "32", "Fizz", "34", "Buzz", "Fizz", "37", "38",
        "Fizz", "Buzz", "41", "Fizz", "43", "44", "FizzBuzz", "46", "47", "Fizz", "49", "Buzz",
        "Fizz", "52", "53", "Fizz", "Buzz", "56", "Fizz", "58", "59", "FizzBuzz", "61", "62",
        "Fizz", "64", "Buzz", "Fizz", "67", "68", "Fizz", "Buzz", "71", "Fizz", "73", "74",
        "FizzBuzz", "76", "77", "Fizz", "79", "Buzz", "Fizz", "82", "83", "Fizz", "Buzz", "86",
        "Fizz", "88", "89", "FizzBuzz", "91", "92", "Fizz", "94", "Buzz", "Fizz", "97", "98",
        "Fizz", "Buzz",
    ];

    models::Person::test_obj_builder()
        .set_first_name(|i| match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".into(),
            (0, _) => "Fizz".into(),
            (_, 0) => "Buzz".into(),
            _ => i.to_string(),
        })
        .with_index(1)
        .build_vec(100)
        .iter()
        .enumerate()
        .for_each(|(i, x)| assert_eq!(expected_results[i], x.first_name));
}

#[test]
fn verify_char() {
    let type_tester = models::TypeTester::test_obj_builder().build();
    assert_eq!('0', type_tester.type_char);
}

#[test]
fn verify_recursive_building() {
    let type_tester = models::TypeTester::test_obj_builder().build();
    assert_eq!("first_name0", type_tester.type_person.first_name);
}

#[test]
fn verify_recursive_building_vec() {
    models::TypeTester::test_obj_builder()
                        .build_vec(100)
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| assert_eq!(format!("first_name{}", i), x.type_person.first_name));
}

#[test]
fn verify_recursive_building_vec_with_setter() {
    let person_builder = models::Person::test_obj_builder()
                                .set_first_name(|i| i.to_string());
    let person_builder = Arc::new(Mutex::new(person_builder));
    models::TypeTester::test_obj_builder()
                        .set_type_person(move |_| person_builder.lock().unwrap().build()) 
                        .build_vec(100)
                        .iter()
                        .enumerate()
                        .for_each(|(i, x)| assert_eq!(i.to_string(), x.type_person.first_name));
}
