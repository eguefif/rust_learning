use json_parser::from_string;
use json_parser::JsonType;

fn main() {
    let json = r#"
{ 
    "key1": "value1",
    "key2": 5,
    "key3": 1.1,
    "key4": 15.13,
    "key5": false,
    "key6": true,
    "key7": {
        "key71": 1,
        "key72": "Hello, world"
    }, 
    "key8": [1, 2, 3]
}"#;
    let j = from_string(json).unwrap();
    let obj = &j["key7"];
    if let JsonType::Object(obj) = obj {
        println!("{:?}", obj);
    }
}
