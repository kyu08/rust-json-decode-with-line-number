use codespan::Files;
use json_spanned_value::{self as jsv, spanned};

fn main() {
    let text = include_str!("demo.json");
    let mut files = Files::new();
    let file = files.add("demo.json", text);

    let example: spanned::Object = jsv::from_str(text).unwrap();
    for (_, v) in example {
        v.as_object().map(|o| {
            for (k, v) in o {
                let line_index = files.line_index(file, k.start() as u32);
                println!("key: {}, line_index: {}", k.as_str(), line_index.number());
                println!("value: {}", v.as_string().unwrap().to_string());
            }
        });
    }
}
