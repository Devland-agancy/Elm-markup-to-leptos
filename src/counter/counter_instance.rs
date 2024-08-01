use super::counter_types::{CounterType, CounterValueType};

#[derive(Debug, Clone)]
pub struct CounterInstance {
    pub counter_type: CounterType,
    pub name: String,
    pub current_value: CounterValueType,
}

// pub trait AssignType<T>  {

// }

impl CounterInstance {
    pub fn new(name: &str, _type: &str) -> Self {
        Self {
            name: name.to_string(),
            counter_type: CounterType::from_str(_type).unwrap(),
            current_value: CounterValueType::from_str(_type),
        }
    }
}
