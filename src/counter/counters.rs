use crate::parser_helpers::{CellType, DataCell};

use super::{
    counter_commands::CommandType, counter_instance::CounterInstance,
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
    pub fn check_scope(&self, block_cell: &DataCell, counter_name: &str, json_tree: &DataCell) {
        let counter = self.counters_list.iter().find(|c| c.name == counter_name);
        assert!(
            !counter.is_none(),
            "Counter {counter_name} Does not exist or is out of scope"
        );

        // we need to see if block_cell is child of element with id counter.scope
        let mut parents_list = Vec::new();
        self.search_el(block_cell, json_tree, &mut parents_list);
        println!("pare {:?}", block_cell.id);
        println!("pare {:?}", counter.unwrap().scope);

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
            CellType::Block(block) => json_tree.id == block_cell.id,
            CellType::Element(el) => {
                parents_list.push(json_tree.id.try_into().unwrap());

                for child in el.children.iter() {
                    if self.search_el(block_cell, child, parents_list) {
                        println!("tre");
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
                        println!("tre");
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
