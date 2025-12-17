#[derive(Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: Value,
}

#[derive(Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl KeyValue {
    pub fn to_define(&self) -> String {
        match &self.value {
            Value::String(s) => format!("-D{}=\"{}\"", self.key.to_uppercase(), s),
            Value::Integer(i) => format!("-D{}={}", self.key.to_uppercase(), i),
            Value::Float(f) => format!("-D{}={}", self.key.to_uppercase(), f),
            Value::Boolean(b) => {
                if *b {
                    format!("-D{}", self.key.to_uppercase())
                } else {
                    "".to_string()
                }
            }
        }
    }

    pub fn to_cli_option(&self) -> String {
        match &self.value {
            Value::String(s) => format!("--{}={}", self.key.replace("_", "-"), s),
            Value::Integer(i) => format!("--{}={}", self.key.replace("_", "-"), i),
            Value::Float(f) => format!("--{}={}", self.key.replace("_", "-"), f),
            Value::Boolean(b) => {
                if *b {
                    format!("--{}", self.key.replace("_", "-"))
                } else {
                    "".to_string()
                }
            }
        }
    }
}
