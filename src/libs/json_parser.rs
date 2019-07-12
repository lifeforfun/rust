use std::collections::HashMap;

type F64 = f64;
type I64 = i64;

enum Number {
    I64(I64),
    F64(F64),
}

enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

const TOKEN_OBJECT_START: &'static str = "{";
const TOKEN_OBJECT_CLOSE: &'static str = "}";
const TOKEN_ARRAY_START: &'static str = "[";
const TOKEN_ARRAY_CLOSE: &'static str = "]";
const TOKEN_SINGLE_QUOTE: &'static str = "\'";
const TOKEN_QUOTE: &'static str = "\"";


/// 匹配name
/// "name" or name
fn parse_name(&s: &str) -> Option<Value::String> {



    Ok(Value::String("name".to_string()))
}

fn parse_literal(&s: &str) -> Option(Value) {
    match &s {
        "null" => Ok(Value::Null),
        "false" => Ok(Value::Bool(false)),
        "true" => Ok(Value::Bool(true)),
        _ => None,
    }
}

fn parse_number(&s: &str) -> Option(Value::Number(Number)) {
    Ok(Value::Number(Number::F64(90 as F64)))
}

fn parse_string(&s: &str) -> Option<String>{

}

fn parse_array() -> Option<Vec<Value>> {}

fn parse_object() -> Option<HashMap<String, Value>> {}

fn ws_end() -> Option(){

}

impl Value {
    fn parse(&s: &str) -> Self{
        let object = Value::Object(HashMap::<String, Value>::new());
        for i in s.to_string().into_bytes().iter() {

        }
        object
    }
}

pub fn test()
{
    let data = r#"
        {}
    "#;
}