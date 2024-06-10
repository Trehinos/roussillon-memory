use roussillon_type_system::value::reference::Reference;
use roussillon_type_system::value::concept::ValueCell;

pub trait Allocator {
    fn allocate(&mut self, cell: ValueCell) -> Reference;
}

pub trait Dereference {
    fn dereference(&self, reference: Reference) -> Option<ValueCell>;
}

#[derive(Clone, Default, Debug)]
pub struct Memory {
    raw: Vec<u8>,
}

impl Memory {
    pub fn len(&self) -> usize { self.raw.len() }
    pub fn is_empty(&self) -> bool { self.raw.is_empty() }
}

impl Allocator for Memory {
    fn allocate(&mut self, cell: ValueCell) -> Reference {
        let address = self.raw.len();
        let borrowed_cell = cell.borrow();
        self.raw.extend_from_slice(&borrowed_cell.raw());
        Reference::new(
            borrowed_cell.data_type().clone(),
            address,
        )
    }
}

impl Dereference for Memory {
    fn dereference(&self, reference: Reference) -> Option<ValueCell> {
        let start = reference.get_address();
        let referenced_type = reference.referenced();
        let end = start + referenced_type.size();
        if end > self.raw.len() { return None; };
        let raw = &self.raw[start..end];
        Some(referenced_type.construct_from_raw(raw).unwrap())
    }
}