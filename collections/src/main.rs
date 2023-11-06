use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(4, 6);
    map.insert(1, 3);

    println!("map: {:?}", map);
    println!("map: {:?}", map.get(&1));
    println!("map: {:?}", map.get(&2));

    let mut map2: HashMap<i32, Vec<i32>> = HashMap::new();
    match map2.entry(5) {
        Entry::Occupied(mut e) => {
            e.get_mut().push(7);
        }
        Entry::Vacant(e) => {
            e.insert(vec![7]);
        }
    };

    println!("{:?}", map2);

    let mut person_map = HashMap::new();
    person_map.insert(Person {
        name: "asdf".to_string(),
    },
    8);

    for a in person_map.keys() {
        println!("  {:?}", a);
    }
}


#[derive(Eq, PartialEq, Debug)]
struct Person {
    name: String,
}
impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
