use json_parser::from_string;

fn main() {
    let json = r#"
{ 
    "key1": "value1",
    "key2": 5,
    "key3": 1.1,
    "key4": 15.13,
    "key5": false,
    "key6": true
}"#;
    let j = from_string(json).unwrap();
    println!("{:?}", j["key1"]);
}
