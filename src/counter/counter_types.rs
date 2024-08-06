use super::counter_instance::CounterInstance;

#[derive(Debug, Clone)]
pub enum CounterType {
    ARABIC,
    ROMAN,
    ALPHABITICAL,
}

#[derive(Debug, Clone, Copy)]
pub enum CounterValueType {
    ARABIC(isize),
    ROMAN(char),
    ALPHABITICAL(char),
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
            "counter" => Self::ARABIC(0),
            "roman_counter" => Self::ROMAN('0'),
            "alphabitical_counter" => Self::ALPHABITICAL('0'),
            _ => Self::ARABIC(0),
        }
    }

    pub fn from_type(counter_type: &CounterType) -> Self {
        match counter_type {
            CounterType::ARABIC => Self::ARABIC(0),
            CounterType::ROMAN => Self::ROMAN('0'),
            CounterType::ALPHABITICAL => Self::ALPHABITICAL('0'),
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
            CounterValueType::ROMAN(roman) => {
                let code_point = *roman as u32;
                if *roman != '0' && code_point < 8560 {
                    "-".to_string()
                } else {
                    roman.to_string()
                }
            }
            CounterValueType::ALPHABITICAL(string) => string.to_string(),
        }
    }
}
