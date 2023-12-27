
#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}


// fn whitespace(input: &str) -> IResult<&str, &str> {
//     let chars = " \t\r\n";
//     take_while(move |c| chars.contains(c))(input)
// }

// number
