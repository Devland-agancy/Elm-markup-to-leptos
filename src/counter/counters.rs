use super::{counter_commands::CommandType, counter_instance::CounterInstance};

#[derive(Debug, Clone)]
pub struct Counters {
    pub counters_list: Vec<CounterInstance>,
}

// we need to get counter props
// look for counter usage

impl Counters {
    pub fn new() -> Counters {
        Counters {
            counters_list: Vec::new(),
        }
    }

    pub fn add_counter(&mut self, counter: CounterInstance) {
        let counter_exists = self.counters_list.iter().any(|c| c.name == counter.name);
        assert!(!counter_exists, "Counter name already used");
        self.counters_list.push(counter);
    }

    pub fn remove_counter(&mut self, counter: &CounterInstance) {
        self.counters_list = self
            .counters_list
            .iter()
            .filter(|c| c.name != counter.name)
            .cloned()
            .collect();
    }

    pub fn execute(&mut self, command_type: CommandType, counter_name: &str) -> Option<String> {
        let counter = self
            .counters_list
            .iter_mut()
            .find(|c| c.name == counter_name);
        assert!(
            !counter.is_none(),
            "Counter {counter_name} Does not exist or is out of scope"
        );
        match command_type {
            CommandType::INC => counter.unwrap().increment(),
            CommandType::DEC => counter.unwrap().decrement(),
            CommandType::ASSIGN => {}
            CommandType::INSERT => {
                return Some(counter.unwrap().current_value.to_string());
            }
        }
        None
    }
}
