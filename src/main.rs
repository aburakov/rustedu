use crate::metanit::chapter_3::{BaoMember, Operations};

mod chapters;
mod metanit;

fn main() {

}

fn get_message(oper: fn(i32, i32) -> i32) -> String {
    println!("{}", oper(1, 3));
    "Hello Alex".to_string()
}

fn ss(a: i32, b: i32) -> i32 {
    a + b
}

fn print_family_member(bao_member: &mut BaoMember) {
    println!("Member: {} {}", bao_member.name, bao_member.age);
}