use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("Article Name"),
        author: String::from("Author Name"),
        paragraph: vec![
            Paragraph {
                name: String::from("Para 1"),
            },
            Paragraph {
                name: String::from("Para 2"),
            },
            Paragraph {
                name: String::from("Para 3"),
            },
        ],
    };

    let json: String = serde_json::to_string(&article).unwrap();

    println!("Json | {}", json);
}
