use super::counter_types::{CounterType, CounterValueType};

#[derive(Debug, Clone)]
pub struct CounterInstance {
    pub counter_type: CounterType,
    pub name: String,
    pub current_value: CounterValueType,
    pub scope: usize,
}

// pub trait AssignType<T>  {

// }

impl CounterInstance {
    pub fn new(name: &str, _type: &str, scope: usize) -> Self {
        Self {
            name: name.to_string(),
            counter_type: CounterType::from_str(_type).unwrap(),
            current_value: CounterValueType::from_str(_type),
            scope,
        }
    }

    pub fn increment(&mut self) {
        match self.counter_type {
            CounterType::ARABIC => {
                self.current_value = match self.current_value {
                    CounterValueType::ARABIC(i) => CounterValueType::ARABIC(i + 1),
                    _ => self.current_value,
                }
            }
            CounterType::ROMAN => {
                self.current_value = match self.current_value {
                    CounterValueType::ROMAN(roman) => {
                        if roman == '0' {
                            CounterValueType::ROMAN('ⅰ')
                        } else {
                            let code_point = roman as u32;
                            let incremented = std::char::from_u32(code_point + 1).unwrap();
                            CounterValueType::ROMAN(incremented)
                        }
                    }
                    _ => self.current_value,
                }
            }
            CounterType::ALPHABITICAL => {}
            _ => {}
        }
    }

    pub fn decrement(&mut self) {
        match self.counter_type {
            CounterType::ARABIC => {
                self.current_value = match self.current_value {
                    CounterValueType::ARABIC(i) => CounterValueType::ARABIC(i - 1),
                    _ => self.current_value,
                }
            }
            CounterType::ROMAN => {
                self.current_value = match self.current_value {
                    CounterValueType::ROMAN(roman) => {
                        if roman == 'i' {
                            CounterValueType::ROMAN('0')
                        } else {
                            let code_point = roman as u32;
                            let incremented = std::char::from_u32(code_point - 1).unwrap();
                            CounterValueType::ROMAN(incremented)
                        }
                    }
                    _ => self.current_value,
                }
            }
            CounterType::ALPHABITICAL => {}
            _ => {}
        }
    }
}
