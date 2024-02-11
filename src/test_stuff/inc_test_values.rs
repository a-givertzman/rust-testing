use log::debug;

use super::test_value::Value;

///
/// Simply iterates through the incrementing value by +1, begining from the initial
#[derive(Debug, Clone)]
pub struct IncTestValues {
    id: String,
    iterations: usize,
    value: i64,
}
///
/// 
impl IncTestValues {
    ///
    /// Simply iterates through the incrementing value by +1, begining from the initial
    pub fn  new(parent: impl Into<String>, initial: i64, iterations: usize) -> Self {
        Self {
            id: format!("{}/IncTestValues", parent.into()),
            iterations,
            value: initial,
        }
    }
    }
///
/// 
impl Iterator for IncTestValues {
    type Item = Value;
    //
    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations > 0 {
            self.iterations -= 1;
            let value = self.value;
            self.value = value + 1;
            debug!("{}.next | Шterations awalible: {}", self.id, self.iterations);
            Some(Value::Int(value))
        } else {
            debug!("{}.next | No more iterations awalible", self.id);
            None
        }
    }
}