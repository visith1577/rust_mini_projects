use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}


fn read_json(raw_json: &str) -> Article{
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}


fn main() {
    let json = r#"
    {
        "article" : "How to work with serde",
        "author" : "Visith",
        "paragraph" : [
            {
                "name": "serde"
            },
            {
                "name": "serde_json"
            },
            {
                "name": "serialize"
            }
        ]
    }"#;

    let parsed: Article = read_json(json);
    println!("\n\n Name of the first paragraph in : {}", parsed.paragraph[0].name);
}
