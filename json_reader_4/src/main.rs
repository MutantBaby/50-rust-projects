use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]

struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json: &str = r#"
        {
            "article": "Work with Json in Rust",
            "author": "Sheru",
            "paragraph": [
                {
                    "name": "Starting Sentence"
                },
                {
                    "name": "Body Sentence"
                },
                {
                    "name": "End Sentence"
                }
            ]
        }
    "#;

    let parsed: Article = read_json_type(json);

    println!("Parsed Value | \n\n{:#?}", parsed);
}

fn read_json_type(json: &str) -> Article {
    serde_json::from_str(json).unwrap()
}
