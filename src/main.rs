extern crate tob;
use tob::tob;

#[derive(tob)]
struct Person {
    first_name: String,
    last_name: String,
    ssn: String,
}

fn main() {
    let p = Person::tob().build();
    println!("hello {:?} ", p.first_name);
    println!("hello {:?} ", p.last_name);
    println!("hello {:?} ", p.ssn);

    Person::tob()
           .build_vec()
           .iter()
           .for_each(|x| println!("Hello {} {} {}", x.first_name, x.last_name, x.ssn));
}
