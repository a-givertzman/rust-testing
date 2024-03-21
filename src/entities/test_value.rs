///
/// Enum wrapper for some elementary types
#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Real(f32),
    Double(f64),
    String(String),
}
impl Value {
    pub fn to_string(&self) -> String {
        match &self {
            Value::Bool(v) => v.to_string(),
            Value::Int(v) => v.to_string(),
            Value::Real(v) => v.to_string(),
            Value::Double(v) => v.to_string(),
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
    /// Returns Double content, otherwise panic
    pub fn as_real(&self) -> f32 {
        match self {
            Value::Real(value) => *value,
            _ => panic!("Value.asFloat | {:?} - is not a Float", self),
        }
    }
    ///
    /// Returns Double content, otherwise panic
    pub fn as_double(&self) -> f64 {
        match self {
            Value::Double(value) => *value,
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
