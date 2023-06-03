#![allow(unused)]

// Import library of Rust
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};


mod numeric;
mod vec;
mod structure;

fn say_name(name: &str){
    println!("{}", name);
}

fn bundle_age(age: u32) -> String {

    if (age < 10){
        return String::from("no");
    }
    return String::from("Fuck you");
}

// fn main() {

//     // Data Type & Variable
//     let sum: i32 = 1+2;

//     println!("{}", sum);

//     let name: String = "Daveat".to_string();

//     say_name(&name);

//     println!("{}", name);

//     // Comparison Operator

//     if 1 == 10 {
//         println!("True");
//     } else {
//         println!("False");
//     }

//     let age = bundle_age(10);


// }

//// Array
// fn main(){
//     let vc = vec::vec_function();
//     println!("{}", vc[0]);
// }

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: i32
}

impl User{

    // pub fn new() -> Self();
}

fn main(){
    // let user = User{name: "Daveat".to_string(), age: 24};

    // let serialized = serde::to_string(&user).unwrap();
}