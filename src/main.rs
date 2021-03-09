extern crate tob;
use tob::tob;

#[derive(tob)]
struct Person {
    first_name: String,
    last_name: String,
    ssn: String,
}

#[derive(tob)]
struct Teacher {
    first_name: String,
    last_name: String,
    ssn: String,
}

struct InnerPob {
    thing : Option<Box<dyn FnMut(usize) -> String>>
}
impl InnerPob {
    pub fn new() -> InnerPob {
        InnerPob {
            thing: None
        }
    }

    pub fn set_thing<F: 'static>(mut self, f: F) -> Self where
        F: Fn(usize) -> String {
            self.thing = Some(Box::new(f));
            self
    }

    pub fn build(&mut self) -> Person {
        let t = self.thing.as_mut().unwrap();
        Person {
            first_name: t(0),
            last_name: t(1),
            ssn: t(2)
        }
    }
}

struct OB<T> {
    phantom: std::marker::PhantomData<T>
}

impl OB<Person> {
    pub fn new() -> InnerPob {
        InnerPob::new()
    }
}

fn main() {
    let mut  x = OB::<Person>::new();
    x = x.set_thing(|i| format!("blaha{}", i).into());
    let result = x.build();
    println!("Hello Test {} {} {}", result.first_name, result.last_name, result.ssn);


    let mut p = ObjectBuilder::<Person>::new();
    let p = p.set_first_name(|i| format!("baa{}", i).into())
             .build();
    println!("Hello Person {} {} {}", p.first_name, p.last_name, p.ssn);

    let mut p = ObjectBuilder::<Person>::new();
    p.build_vec()
     .iter()
     .for_each(|x| println!("Hello Persons {} {} {}", x.first_name, x.last_name, x.ssn));

    let mut p = ObjectBuilder::<Teacher>::new();
    let p = p.build();
    println!("Hello Teacher {} {} {}", p.first_name, p.last_name, p.ssn);
}
