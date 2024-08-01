#[derive(Debug, Clone)]
pub enum CounterType {
    ARABIC,
    ROMAN,
    ALPHABITICAL,
}

#[derive(Debug, Clone, Copy)]
pub enum CounterValueType {
    NUMBER(usize),
    STRING(char),
}

impl CounterType {
    pub fn is_valid(string: &str) -> bool {
        string == "counter" || string == "roman_counter" || string == "alphabitical_counter"
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "counter" => Some(Self::ARABIC),
            "roman_counter" => Some(Self::ROMAN),
            "alphabitical_counter" => Some(Self::ALPHABITICAL),
            _ => None,
        }
    }
}

impl CounterValueType {
    pub fn from_str(string: &str) -> Self {
        match string {
            "counter" => Self::NUMBER(0),
            "roman_counter" => Self::STRING('ⅰ'),
            "alphabitical_counter" => Self::STRING('a'),
            _ => Self::NUMBER(0),
        }
    }

    pub fn from_type(counter_type: &CounterType) -> Self {
        match counter_type {
            CounterType::ARABIC => Self::NUMBER(0),
            CounterType::ROMAN => Self::STRING('ⅰ'),
            CounterType::ALPHABITICAL => Self::STRING('a'),
            _ => Self::NUMBER(0),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CounterValueType::NUMBER(number) => number.to_string(),
            CounterValueType::STRING(string) => string.to_string(),
        }
    }

    pub fn increment(&mut self, counter_type: &CounterType) {
        match counter_type {
            CounterType::ARABIC => {
                *self = match *self {
                    CounterValueType::NUMBER(i) => CounterValueType::NUMBER(i + 1),
                    _ => *self,
                }
            }
            CounterType::ROMAN => {
                *self = match self {
                    CounterValueType::STRING(roman) => {
                        let code_point = *roman as u32;
                        // let start = 0x2170; // ⅰ (Roman Numeral Small One)
                        // let end = 0x217F;

                        let incremented = std::char::from_u32(code_point + 1).unwrap();

                        CounterValueType::STRING(incremented)
                    }
                    _ => *self,
                }
            }
            CounterType::ALPHABITICAL => {}
            _ => {}
        }
    }
}
