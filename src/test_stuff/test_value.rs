///
/// Enum wrapper for some elementary types
#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}
impl Value {
    pub fn to_string(&self) -> String {
        match &self {
            Value::Bool(v) => v.to_string(),
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::String(v) => v.to_string(),
        }
    }
    ///
    /// Returns Bool content, otherwise panic
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(value) => *value,
            _ => panic!("Value.asBool | {:?} - is not an Bool", self),
        }
    }
    ///
    /// Returns Int content, otherwise panic
    pub fn as_int(&self) -> i64 {
        match self {
            Value::Int(value) => *value,
            _ => panic!("Value.asInt | {:?} - is not an Int", self),
        }
    }
    ///
    /// Returns Float content, otherwise panic
    pub fn as_float(&self) -> f64 {
        match self {
            Value::Float(value) => *value,
            _ => panic!("Value.asFloat | {:?} - is not a Float", self),
        }
    }
    ///
    /// Returns String content, otherwise panic
    pub fn as_string(&self) -> String {
        match self {
            Value::String(value) => value.clone(),
            _ => panic!("Value.asString | {:?} - is not a String", self),
        }
    }
}
