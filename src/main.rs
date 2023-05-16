use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragtaph{
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragtaph>
}

fn main() {
    let json = r#"
    {
        "article": "How to work with json in Rust",
        "author": "Morten",
        "paragraph":[
            {
                "name": "Start sentences"
            },
            {
                "name": "Body of paragraph"
            },
            {
                "name": "End of paragraph"
            }
        ]
    }"#;
        let parsed: Article = read_json_typed(json);

        println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);


    fn read_json_typed(raw_json: &str) -> Article {
        let parsed: Article = serde_json::from_str(raw_json).unwrap();
        return parsed
    }
}
