# Json parser

This is a learning project to deepen my knowledge on Rust.
This is not to use in a real project!

## Todo
- [x] implement from_json to be used for every type and struct
- [x] implement to_json and an algo that serialize an object


## Architecture

There are two way to use this library:
1 using the generic JsonType
2 implementing Serialize and Deserialize to a struct

The central type is `JsonType`. This type can be directly used when a user choose 1 to manipulate Json. This type is use in the implentation of `Deserialize` to create an object.

### The parsing logic
The parsing logic happend in two times
1. Tokenizer: return a vec of Token
2. Parser: transform the vec of Token into a JsonType with basic type.
