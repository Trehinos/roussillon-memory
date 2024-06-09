use roussillon_type_system::value::concept::ValueCell;
use roussillon_type_system::value::reference::Reference;

pub trait Allocator {
    fn alloc_cell(&mut self, cell: ValueCell) -> Reference;
}

#[derive(Clone, Default, Debug)]
pub struct AllocOnly {
    raw: Vec<u8>,
}

impl Allocator for AllocOnly {
    fn alloc_cell(&mut self, cell: ValueCell) -> Reference {
        let borrowed_cell = cell.borrow();
        self.raw.extend_from_slice(&borrowed_cell.raw());
        Reference::new(
            borrowed_cell.data_type().clone(),
            self.raw.len() - 1,
        )
    }
}