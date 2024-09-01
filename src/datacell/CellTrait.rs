use super::Datacell::DataCell;

pub trait Cell<T> {
    fn init_cell(id: usize, parent_id: usize, s: T) -> DataCell;
    fn push_cell(parent: &mut DataCell, id: usize, s: T);
    fn add_cell(add_to: &mut DataCell, parent_id: usize, id: usize, text: T);
}
