use numerals::roman::Roman;

#[derive(Debug, Clone)]
pub enum CounterType {
    ARABIC,
    ROMAN,
    ALPHABITICAL,
}

#[derive(Debug, Clone)]
pub enum CounterValueType {
    ARABIC(isize),
    ROMAN(String),
    ALPHABITICAL(String),
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
    fn is_valid(&self, value: &str) -> bool {
        match self {
            CounterValueType::ARABIC(_) => value.parse::<isize>().is_ok(),
            CounterValueType::ROMAN(roman) => {
                let roman = Roman::parse(roman);
                roman.is_some()
            }
            CounterValueType::ALPHABITICAL(alpha) => alpha.chars().all(|c| {
                let code_point = c as u32;
                code_point >= 65
            }),
        }
    }

    fn default_value(&self, value: Option<&str>) -> Self {
        match self {
            CounterValueType::ARABIC(_) => {
                if let Some(value) = value {
                    if CounterValueType::ARABIC(0).is_valid(value) {
                        Self::ARABIC(value.parse::<isize>().unwrap())
                    } else {
                        panic!("Wrong Arabic counter value")
                    }
                } else {
                    Self::ARABIC(0)
                }
            }
            CounterValueType::ROMAN(_) => {
                if let Some(default_value) = value {
                    if CounterValueType::ROMAN("".to_string()).is_valid(default_value) {
                        Self::ROMAN(default_value.to_string())
                    } else {
                        panic!("Wrong Roman counter value")
                    }
                } else {
                    Self::ROMAN("0".to_string())
                }
            }
            CounterValueType::ALPHABITICAL(_) => {
                if let Some(default_value) = value {
                    if CounterValueType::ALPHABITICAL("".to_string()).is_valid(default_value) {
                        Self::ALPHABITICAL(default_value.to_string())
                    } else {
                        panic!("Wrong Aphabitical counter value")
                    }
                } else {
                    Self::ALPHABITICAL("0".to_string())
                }
            }
        }
    }

    pub fn from_str(string: &str, default_value: Option<&str>) -> Self {
        match string {
            "counter" => CounterValueType::ARABIC(0).default_value(default_value),
            "roman_counter" => CounterValueType::ROMAN("".to_string()).default_value(default_value),
            "alphabitical_counter" => {
                CounterValueType::ALPHABITICAL("".to_string()).default_value(default_value)
            }
            _ => Self::ARABIC(0),
        }
    }

    pub fn from_type(counter_type: &CounterType, default_value: Option<&str>) -> Self {
        match counter_type {
            CounterType::ARABIC => CounterValueType::ARABIC(0).default_value(default_value),
            CounterType::ROMAN => {
                CounterValueType::ROMAN("".to_string()).default_value(default_value)
            }
            CounterType::ALPHABITICAL => {
                CounterValueType::ALPHABITICAL("".to_string()).default_value(default_value)
            }
            _ => Self::ARABIC(0),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CounterValueType::ARABIC(number) => {
                if *number < 0 {
                    "-".to_string()
                } else {
                    number.to_string()
                }
            }
            CounterValueType::ROMAN(roman) => roman.to_string(),
            CounterValueType::ALPHABITICAL(string) => string.to_string(),
        }
    }
}
