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
    let json = r#"
    {
        "title": "My Article",
        "author": "John Doe",
        "paragraphs": [
            {
                "text": "Lorem ipsum dolor sit amet."
            },
            {
                "text": "Consectetur adipiscing elit."
            }
        ]
    }"#;

    let article: Article = serde_json::from_str(json).unwrap();
    println!("{}", serde_json::to_string_pretty(&article).unwrap());
}
