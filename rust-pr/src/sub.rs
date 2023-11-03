use std::io::{self, Read};
use std::fs::File;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} : {}", self.headline, self.author, self.content)
    }
}


pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    println!("Reading usernames...");

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

pub struct User {
    pub name: String,
    pub id: u32,
    pub email: String,
    pub sign_in: bool,
}


pub fn first_word(s: &String) -> usize {
    let sbytes = s.as_bytes();

    for (i, &item) in sbytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
} 

pub fn first_word_slice(s: &String) -> &str {
    let sbytes = s.as_bytes();

    for (i, &item) in sbytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    return &s[..];
}

pub fn set_signature(user: &mut User) {
    user.sign_in = true;
}