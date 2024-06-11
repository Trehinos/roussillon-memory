use roussillon_type_system::value::concept::ValueCell;
use roussillon_type_system::value::reference::Reference;

use crate::region::{Allocator, Dereference, Region};

#[derive(Clone, Debug)]
pub struct StackReference {
    level: usize,
    reference: Reference,
}

impl StackReference {
    pub fn reference(&self) -> &Reference {
        &self.reference
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

    pub fn pop(&mut self) -> Option<Region> {
        self.raw.pop()
    }
}

impl Allocator<StackReference> for Stack {
    fn allocate(&mut self, cell: ValueCell) -> StackReference {
        let mut current = self.pop().unwrap();
        let level = self.raw.len();
        let reference = current.allocate(cell);
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
