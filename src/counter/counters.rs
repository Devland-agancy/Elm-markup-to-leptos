use crate::parser_helpers::{CellType, DataCell};

use super::{
    counter_commands::CommandType, counter_instance::CounterInstance, counter_types::CounterType,
    handle_instance::HandleInstance,
};

#[derive(Debug, Clone)]
pub struct Counters {
    pub counters_list: Vec<CounterInstance>,
    pub handles_list: Vec<HandleInstance>,
}

// we need to get counter props
// look for counter usage

impl Counters {
    pub fn new() -> Counters {
        Counters {
            counters_list: Vec::new(),
            handles_list: Vec::new(),
        }
    }

    pub fn get_counters_from_json(&mut self, json: &DataCell) {
        match &json.cell_type {
            CellType::Element(el) => {
                el.props.iter().for_each(|prop| {
                    if CounterType::is_valid(&prop.key) {
                        let mut value_splits = prop.value.split(" ");
                        let counter_name = value_splits.next().expect("Counter must have a name");

                        let default_value = value_splits.next();
                        // create new counter
                        self.add_counter(CounterInstance::new(
                            counter_name,
                            &prop.key,
                            json.id,
                            default_value,
                        ));
                    }
                });
                el.children
                    .iter()
                    .for_each(|child| self.get_counters_from_json(child))
            }
            CellType::Root(root) => root
                .children
                .iter()
                .for_each(|child| self.get_counters_from_json(child)),
            _ => (),
        }
    }

    pub fn add_counter(&mut self, counter: CounterInstance) {
        let counter_exists = self.counters_list.iter().any(|c| c.name == counter.name);
        assert!(!counter_exists, "Counter name already used");
        self.counters_list.push(counter);
    }

    pub fn add_handle(&mut self, handle: HandleInstance) {
        let handle_exists = self.handles_list.iter().any(|c| c.name == handle.name);
        assert!(!handle_exists, "handle {} already assigned", handle.name);
        self.handles_list.push(handle);
    }

    pub fn remove_counter(&mut self, counter: &CounterInstance) {
        self.counters_list = self
            .counters_list
            .iter()
            .filter(|c| c.name != counter.name)
            .cloned()
            .collect();
    }

    pub fn execute(
        &mut self,
        command_type: CommandType,
        counter_name: &str,
        handle_name: &Option<String>,
    ) -> Option<String> {
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
            CommandType::ASSIGN => {
                if let Some(handle_name) = handle_name {
                    let handle_exists = self.handles_list.iter().any(|c| &c.name == handle_name);
                    assert!(!handle_exists, "handle {} already assigned", handle_name);
                    self.handles_list.push(HandleInstance {
                        name: handle_name.to_string(),
                        value: counter.unwrap().current_value.to_string(),
                    });
                }
            }
            CommandType::INSERT => {
                return Some(counter.unwrap().current_value.to_string());
            }
        }
        None
    }

    pub fn check_scope(&self, block_cell: &DataCell, counter_name: &str, json_tree: &DataCell) {
        let counter = self.counters_list.iter().find(|c| c.name == counter_name);
        assert!(
            !counter.is_none(),
            "Counter {counter_name} Does not exist or is out of scope"
        );

        // we need to see if block_cell is child of element with id counter.scope
        let mut parents_list = Vec::new();
        self.search_el(block_cell, json_tree, &mut parents_list);

        assert!(
            parents_list.contains(&counter.unwrap().scope),
            "Counter {counter_name} is out of scope"
        );
    }

    fn search_el(
        &self,
        block_cell: &DataCell,
        json_tree: &DataCell,
        parents_list: &mut Vec<usize>,
    ) -> bool {
        match &json_tree.cell_type {
            CellType::Block(_) => json_tree.id == block_cell.id,
            CellType::Element(el) => {
                parents_list.push(json_tree.id.try_into().unwrap());

                for child in el.children.iter() {
                    if self.search_el(block_cell, child, parents_list) {
                        return true;
                    }
                }

                parents_list.pop();
                return false;
            }
            CellType::Root(el) => {
                parents_list.push(json_tree.id.try_into().unwrap());

                for child in el.children.iter() {
                    if self.search_el(block_cell, child, parents_list) {
                        return true;
                    }
                }

                parents_list.pop();
                return false;
            }
            _ => false,
        }
    }
}
