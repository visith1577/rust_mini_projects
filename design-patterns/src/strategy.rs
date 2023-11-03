use std::collections::HashMap;

type Data = HashMap<String, u32>;

pub trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}

pub struct Report;

impl Report {
    pub fn generate<T: Formatter>(g: T, s: &mut String) {
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        g.format(&data, s);
    }
}

 