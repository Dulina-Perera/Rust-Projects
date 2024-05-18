use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct Paragraph {
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Article {
    title: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        title: String::from("My Article"),
        author: String::from("John Doe"),
        paragraphs: vec![
            Paragraph {
                text: String::from("Lorem ipsum dolor sit amet."),
            },
            Paragraph {
                text: String::from("Consectetur adipiscing elit."),
            },
        ],
    };

    let json = serde_json::to_string_pretty(&article).unwrap();
    println!("{}", json);
}