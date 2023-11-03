mod sub;
mod calc;
mod design;

use std::collections::HashMap;
use std::fs::File;



#[derive(Debug)]
enum IpAddr<T> {
    IPV4(T),
}


struct Color(i32, i32, i32);

impl Color {
    fn sum(&self) -> i32 {
        return self.0 + self.2;
    }
}


fn main() {

    // let _file = match File::open("hello.txt") {
    //     Ok(f) => f,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(f) => f,
    //             Err(e) => panic!("Error {:?} creating file", e),
    //         },
    //         other_error => {
    //             panic!("Error: {:?}", other_error);
    //         }
    //     }
    // }; 

    sub::read_username_from_file().expect("Error at read username");

    let _file = File::open("hello.txt")
                        .expect("File should be included");

    let mut scores:HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Math"), 80);
    scores.insert(String::from("RAD"), 83);

    let subject_name = String::from("Meth");
    println!("score for {} is {}", subject_name, scores.get(&subject_name).copied().unwrap_or(0));

    let mut vec:Vec<i32> = Vec::new();

    vec.push(35);
    vec.push(36);
    vec.push(37);
    vec.push(38);

    let s = String::from("Hellow");
    let s2 = "wow".to_string() + &s;

    let s3 = format!("{s} - {s2}");

    println!("{s2} ==>> {s3}");

    let home = IpAddr::IPV4(String::from("127.0.0.0"));

    dbg!(home);

    let mut user = sub::User {
        name: String::from("Visith"),
        id: 34,
        email: String::from("visithkumaapperuma@gmail.com"),
        sign_in: false,
    };

    let id = match user.id {
        10 => 100,
        20 => 200,
        30 => 300,
        _ => 1000
    };
    
    println!("{} is the name of user with {} @ id: {}", user.name, user.email, id);

    let col = Color(12, 0, 0);

    println!("{} @sum", col.sum());

    let user1 = sub::User {
        name: String::from("Chamika"),
        email: String::from("visithkumaapperuma@gmail.com"),
        ..user
    };

    println!("{} is the id of user1", user1.id);

    sub::set_signature(&mut user);

    let s = String::from("hello RustCC");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    let word = sub::first_word(&s);
    println!("{} word size", word);
    // s.clear();
    let slice = sub::first_word_slice(&s);
    println!("{} is the first word", slice);
    // variables r1 and r2 will not be used after this point

    println!("consec vowel 3? {}", design::three_vowels("haeol"));

}

