use roussillon_type_system::typing::concept::DataType;
use roussillon_type_system::typing::primitive::Primitive;
use roussillon_type_system::value::boolean::Boolean;
use roussillon_type_system::value::byte::Bytes;
use roussillon_type_system::value::concept::ValueCell;
use roussillon_type_system::value::reference::Reference;

pub trait Allocator {
    fn alloc_cell(&mut self, cell: ValueCell) -> Reference;
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
    fn alloc_cell(&mut self, cell: ValueCell) -> Reference {
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
        Some(match referenced_type.typename() {
            s if s == Primitive::Byte.typename() || s == Primitive::Bytes(raw.len()).typename() =>
                Bytes::from(raw).to_cell(),
            s if s == Primitive::Boolean.typename() => Boolean::from(raw).to_cell(),
            s => panic!("Unimplemented dereference for type {}", s)
        })
    }
}