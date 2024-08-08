use super::counter_types::{CounterType, CounterValueType};
use leptos::server_fn::default;
use numerals::roman::Roman;

#[derive(Debug, Clone)]
pub struct CounterInstance {
    pub counter_type: CounterType,
    pub name: String,
    pub current_value: CounterValueType,
    pub scope: usize,
}

impl CounterInstance {
    pub fn new(name: &str, _type: &str, scope: usize, default_value: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            counter_type: CounterType::from_str(_type).unwrap(),
            current_value: CounterValueType::from_str(_type, default_value),
            scope,
        }
    }

    pub fn increment(&mut self) {
        match self.counter_type {
            CounterType::ARABIC => {
                if let CounterValueType::ARABIC(i) = self.current_value {
                    self.current_value = CounterValueType::ARABIC(i + 1)
                }
            }
            CounterType::ROMAN => {
                if let CounterValueType::ROMAN(roman) = &self.current_value {
                    self.current_value = if roman == "0" {
                        CounterValueType::ROMAN("i".to_string())
                    } else {
                        let mut num: i16 =
                            Roman::parse(&roman).expect("unvalid roman chars").value();
                        num += 1;
                        let incremented_roman = format!("{:x}", Roman::from(num));
                        CounterValueType::ROMAN(incremented_roman)
                    }
                }
            }
            CounterType::ALPHABITICAL => {}
            _ => {}
        }
    }

    pub fn decrement(&mut self) {
        match self.counter_type {
            CounterType::ARABIC => {
                if let CounterValueType::ARABIC(i) = self.current_value {
                    self.current_value = CounterValueType::ARABIC(i - 1)
                }
            }
            CounterType::ROMAN => {
                if let CounterValueType::ROMAN(roman) = &self.current_value {
                    self.current_value = if roman == "i" {
                        CounterValueType::ROMAN("0".to_string())
                    } else {
                        if let Some(paresed) = Roman::parse(&roman) {
                            let mut num: i16 = paresed.value();
                            num -= 1;
                            let incremented_roman = format!("{:x}", Roman::from(num));
                            CounterValueType::ROMAN(incremented_roman)
                        } else {
                            CounterValueType::ROMAN("-".to_string())
                        }
                    }
                }
            }
            CounterType::ALPHABITICAL => {}
            _ => {}
        }
    }
}
