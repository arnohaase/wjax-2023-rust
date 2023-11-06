fn main() {
    let mut p = Person {
        name: "uninitialized",
        age: 123,
    };

    {
        let the_name = "Max Mustermann".to_string();
        p = Person {
            name: &the_name,
            age: 22,
        };
    }

    println!("after block");
    println!("{:?}", p);
    // println!("{}", p.age);
}


#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}
