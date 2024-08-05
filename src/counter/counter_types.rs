use super::counter_instance::CounterInstance;

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
            "roman_counter" => Self::STRING('i'),
            "alphabitical_counter" => Self::STRING('a'),
            _ => Self::NUMBER(0),
        }
    }

    pub fn from_type(counter_type: &CounterType) -> Self {
        match counter_type {
            CounterType::ARABIC => Self::NUMBER(0),
            CounterType::ROMAN => Self::STRING('i'),
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
}
