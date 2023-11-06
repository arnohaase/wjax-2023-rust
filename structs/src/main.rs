use std::fmt::{Debug, Display, Formatter, Write};

fn main() {
    let p = Person {
        name: "Max Mustermann".to_string(),
        age: 56,
        gender: Gender::Male,
    };
    let p = Person::new("Max Mustermann", 56);

    println!("name: {:?}", p.name);
    println!("p: {:?}", p);
    println!("p: {}", p);
    println!("is_minor: {}", p.is_minor());
    println!("is_minor: {}", Person::is_minor(&p));

    // p.set_name("someone else");

    let mut p2 = p.clone();
    p2.age = 87;
    p2.set_name("someone else");

    println!("p == p2: {}", p == p2);

    dump(p2);
}

#[derive(Debug, PartialEq, Clone)]
enum Gender {
    Male,
    Female,
    Other {
        description: String,
    },
}
impl Gender {
    fn as_char(&self) -> char {
        match self {
            Gender::Male => 'M',
            Gender::Female => 'F',
            Gender::Other { description } => description.chars().next().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
}
impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
            gender: Gender::Other { description: "".to_string() }
        }
    }

    fn is_minor(&self) -> bool {
        self.age < 18
    }

    fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

trait Named {
    fn get_name(&self) -> &str;
}

fn dump<T: Named + Debug>(o: T) {
    println!("{:?} has name {:?}", o, o.get_name());
}

impl Named for Person {
    fn get_name(&self) -> &str {
        &self.name
    }
}