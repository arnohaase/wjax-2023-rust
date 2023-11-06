fn main() {
    println!("Hello, world {}!", "ihr alle");

    let mut a= 1;
    for _ in 0..5 {
        a *= 10;
        println!("a={}", a);
    }

    let mut b = [1u64, 2u64];
    b[0] = 22;
    b = [0, 55];
    println!("b={:?}", b);

    let mut c = Vec::new();
    c = vec![6, 5, 4];

    c.push(99);
    c.push(123);
    println!("popped: {:?}", c.pop());
    println!("c={:?}", c);

    let mut s = "Halli hallo".to_string();
    s.push_str(" auf der W-JAX");
    println!("s={}", s);

    let name = "W-JAX".to_string();
    println!("{}", greeting(&name));
    println!("{}", greeting(&name));
    println!("{}", greeting("ihr alle"));
}

fn greeting(name: &str) -> String {
    // name.push_str("abc");
    format!("Hallo {}", name)
}

fn greeting2(name: Option<&str>) -> String {
    match name {
        Some(s) => format!("Hallo {}", s),
        None => "Hallo Fremder".to_string(),
    }
}

#[cfg(test)]
mod test {
    use crate::greeting2;

    #[test]
    fn test_greeting2() {
        assert_eq!("Hallo Fremder", greeting2(None));
        assert_eq!("Hallo Du", greeting2(Some("Du")));
    }
}