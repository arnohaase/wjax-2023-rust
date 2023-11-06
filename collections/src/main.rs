fn main() {
    let a = vec![1, 2, 5, 4, 7];

    for el in &a {
        println!("{}", el);
    }
    // for el in a {
    //     println!("{}", el);
    // }

    a.iter()
        .filter(|e| *e%2 == 1)
        .map(|el| {
            let a = el * 2 + 1;
            a
        })
        .for_each(|el| println!("modified: {}", el));

    let slice: &[i32] = &a[..3];

}


