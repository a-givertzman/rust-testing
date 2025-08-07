///
/// Enum wrapper for some elementary types
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Real(f32),
    Double(f64),
    String(String),
    Bytes(Vec<u8>),
}
impl Value {
    pub fn to_string(&self) -> String {
        match &self {
            Value::Bool(v) => v.to_string(),
            Value::Int(v) => v.to_string(),
            Value::Real(v) => v.to_string(),
            Value::Double(v) => v.to_string(),
            Value::String(v) => v.to_string(),
            Value::Bytes(v) => String::from_utf8_lossy(v).into_owned(),
        }
    }
    ///
    /// Returns Bool content, otherwise panic
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(value) => *value,
            _ => panic!("Value.as_bool | {:?} - is not an Bool", self),
        }
    }
    ///
    /// Returns Int content, otherwise panic
    pub fn as_int(&self) -> i64 {
        match self {
            Value::Int(value) => *value,
            _ => panic!("Value.as_int | {:?} - is not an Int", self),
        }
    }
    ///
    /// Returns Double content, otherwise panic
    pub fn as_real(&self) -> f32 {
        match self {
            Value::Real(value) => *value,
            _ => panic!("Value.as_real | {:?} - is not a Real", self),
        }
    }
    ///
    /// Returns Double content, otherwise panic
    pub fn as_double(&self) -> f64 {
        match self {
            Value::Double(value) => *value,
            _ => panic!("Value.as_double | {:?} - is not a Double", self),
        }
    }
    ///
    /// Returns String content, otherwise panic
    pub fn as_string(&self) -> String {
        match self {
            Value::String(value) => value.clone(),
            _ => panic!("Value.as_string | {:?} - is not a String", self),
        }
    }
    ///
    /// Returns Bytes content, otherwise panic
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Value::Bytes(value) => value.clone(),
            _ => panic!("Value.as_bytes | {:?} - is not a Bytes", self),
        }
    }
}
