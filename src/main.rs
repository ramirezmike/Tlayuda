#[derive(Debug)]
struct TemplateExample {
    first_name: String,
    last_name: String,
}

impl TemplateExample {
    pub fn tlayuda() -> ExampleBuilder {
        ExampleBuilder::new()
    }
}

struct ExampleBuilder {
    first_name: Box<dyn FnMut(usize) -> String>,
    last_name: Box<dyn FnMut(usize) -> String>,
}
impl ExampleBuilder {
    pub fn new() -> ExampleBuilder {
        ExampleBuilder {
            first_name: Box::new(|x| format!("first_name{}", x).into()),
            last_name: Box::new(|x| format!("last_name{}", x).into()),
        }
    }

    pub fn set_first_name<F: 'static>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> String,
    {
        self.first_name = Box::new(f);
        self
    }

    pub fn _set_last_name<F: 'static>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> String,
    {
        self.last_name = Box::new(f);
        self
    }

    pub fn build(&mut self) -> TemplateExample {
        let first_name = self.first_name.as_mut();
        let last_name = self.last_name.as_mut();

        TemplateExample {
            first_name: first_name(0),
            last_name: last_name(1),
        }
    }
}

fn main() {
    // this is a scratch pad example to model macro output after
    let mut example = TemplateExample::tlayuda();
    example = example.set_first_name(|i| format!("Example{}", i).into());
    println!("{:?}", example.build());
}
