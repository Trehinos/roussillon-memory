use std::cell::RefCell;
use std::rc::Rc;
use roussillon_type_system::types::concept::{DataType, Type};
use roussillon_type_system::value::concept::{DataValue, ValueCell};
use roussillon_type_system::value::error::TypeResult;
use roussillon_type_system::value::reference::Reference;

use crate::region::{Allocator, Dereference, Region};

pub struct StackReferenceType {
    t: Type,
}

impl StackReferenceType {
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for StackReferenceType {
    fn size(&self) -> usize {
        16
    }

    fn typename(&self) -> String {
        format!("$&{}", self.t)
    }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        let (raw_generation, raw_reference) = raw.split_at(8);
        let level = usize::from_be_bytes(raw_generation.try_into().unwrap());
        let raw_reference = usize::from_be_bytes(raw_reference.try_into().unwrap());
        let reference = Reference::new(self.t.clone(), raw_reference);
        Ok(StackReference { level, reference }.to_cell())
    }
}

#[derive(Clone, Debug)]
pub struct StackReference {
    level: usize,
    reference: Reference,
}

impl StackReference {
    pub fn reference(&self) -> &Reference {
        &self.reference
    }
    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
}

impl DataValue for StackReference {
    fn data_type(&self) -> Type {
        StackReferenceType { t: self.reference.data_type() }.to_rc()
    }

    fn raw(&self) -> Vec<u8> {
        let mut raw = Vec::new();
        raw.extend_from_slice(&self.level.to_be_bytes());
        raw.extend_from_slice(&self.reference.raw());
        raw
    }

    fn set(&mut self, raw: &[u8]) {
        let (raw_level, raw_reference) = raw.split_at(8);
        self.level = usize::from_be_bytes(raw_level.try_into().unwrap());
        let raw_reference = usize::from_be_bytes(raw_reference.try_into().unwrap());
        self.reference = Reference::new(self.reference.referenced().clone(), raw_reference);
    }
}

#[derive(Default, Debug)]
pub struct Stack {
    raw: Vec<Region>,
}

impl Stack {
    pub fn new() -> Self { Stack { raw: Vec::new() } }

    pub fn push(&mut self, region: Region) {
        self.raw.push(region)
    }
}

impl Allocator<StackReference> for Stack {
    fn allocate(&mut self, cell: ValueCell) -> StackReference {
        let mut current = self.pop().unwrap();
        let level = self.raw.len();
        let reference = current.allocate(cell);
        self.push(current);
        StackReference { level, reference }
    }
}

impl Dereference<StackReference> for Stack {
    fn dereference(&self, reference: StackReference) -> Option<ValueCell> {
        if reference.level >= self.raw.len() {
            None
        } else {
            self.raw[reference.level].dereference(reference.reference)
        }
    }

    fn validate(&self, reference: &StackReference) -> bool {
        reference.level < self.raw.len()
    }
}
