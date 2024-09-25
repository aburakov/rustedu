pub fn simple_collections() {
    let tuple_example = ("Alex Burakov".to_string(), 1985);
    let array_example: [String; 4] = ["Alex".to_string(), "Daria".to_string(), "Max".to_string(), "Daniil".to_string()];

    println!("{}", tuple_example.0);
    for name in array_example {
        print!("{} ", name);
    }
}

pub struct BaoMember {
    pub name: String,
    pub age: i32,
}

pub fn calc(operations: Operations) {
    match operations {
        Operations::Summ(x, y) => println!("Summ {x} {y}"),
        Operations::Negation(x, y) => println!("Negation {x} {y}")
    }
}

pub enum Operations {
    Summ(i32, i32),
    Negation(i32, i32),
}

pub fn pattern_test() {
    let test_tuple: (&str, i32) = ("Vospi", 2024);

    match test_tuple {
        ("Vospi", 2024) => println!("Correct"),
        _ => println!("Dont know")
    }
}