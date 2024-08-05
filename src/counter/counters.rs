use super::counter_instance::CounterInstance;

#[derive(Debug)]
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
}