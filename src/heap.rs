use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use roussillon_type_system::types::concept::{DataType, Type};
use roussillon_type_system::value::concept::{DataValue, ValueCell};
use roussillon_type_system::value::error::TypeResult;
use roussillon_type_system::value::reference::Reference;
use crate::region::{Allocator, Dereference, Region, DroppableRegion};

pub struct HeapReferenceType {
    generation: usize,
    t: Type
}

impl HeapReferenceType {
    pub fn to_rc(self) -> Rc<Self> { Rc::new(self) }
}

impl DataType for HeapReferenceType {
    fn size(&self) -> usize {
        16
    }

    fn typename(&self) -> String {
        format!("@{}&{}", self.generation, self.t)
    }

    fn construct_from_raw(&self, raw: &[u8]) -> TypeResult<ValueCell> {
        let (raw_generation, raw_reference) = raw.split_at(8);
        let generation = usize::from_be_bytes(raw_generation.try_into().unwrap());
        let raw_reference = usize::from_be_bytes(raw_reference.try_into().unwrap());
        let reference = Reference::new(self.t.clone(), raw_reference);
        Ok(HeapReference{ generation, reference }.to_cell())
    }
}

#[derive(Clone, Debug)]
pub struct HeapReference {
    generation: usize,
    reference: Reference,
}

impl HeapReference {
    pub fn reference(&self) -> &Reference {
        &self.reference
    }

    pub fn generation(&self) -> usize { self.generation }
    pub fn to_cell(self) -> ValueCell { Rc::new(RefCell::new(self)) }
}

impl DataValue for HeapReference {
    fn data_type(&self) -> Type {
        HeapReferenceType{ generation: self.generation, t: self.reference.referenced().clone() }.to_rc()
    }

    fn raw(&self) -> Vec<u8> {
        let mut raw = Vec::new();
        raw.extend_from_slice(&self.generation.to_be_bytes());
        raw.extend_from_slice(&self.reference.raw());
        raw
    }

    fn set(&mut self, raw: &[u8]) {
        let (raw_generation, raw_reference) = raw.split_at(8);
        self.generation = usize::from_be_bytes(raw_generation.try_into().unwrap());
        let raw_reference = usize::from_be_bytes(raw_reference.try_into().unwrap());
        self.reference = Reference::new(self.reference.referenced().clone(), raw_reference);
    }
}

#[derive(Default)]
pub struct Heap {
    raw: Vec<DroppableRegion>,
    current: Option<usize>,
}

impl Heap {
    pub fn new() -> Self { Heap { raw: Vec::new(), current: None } }
    pub fn current_generation(&self) -> Option<usize> { self.current }
    pub fn next_generation(&mut self) -> &Region {
        self.current = match self.current {
            None => Some(0),
            Some(_) => Some(self.raw.len()), // current size IS next index
        };
        self.raw.push(DroppableRegion::Alive(Region::default()));
        self.raw.last().unwrap().unwrap() // Very confident unwrap()s
    }
    pub fn clear(&mut self, generation: usize) { self.raw[generation] = DroppableRegion::Dropped; }
    pub fn is_alive(&self, generation: usize) -> bool { self.raw[generation].is_alive() }
}

impl Allocator<HeapReference> for Heap {
    fn allocate(&mut self, cell: ValueCell) -> HeapReference {
        let current = self.current_generation().unwrap_or_else(|| {
            self.next_generation();
            self.current_generation().unwrap()
        });
        if let Some(DroppableRegion::Alive(region)) = self.raw.get_mut(current) {
            let r = region.allocate(cell);
            HeapReference { generation: self.current.unwrap(), reference: r }
        } else {
            panic!("Invalid region in heap.")
        }
    }
}

impl Dereference<HeapReference> for Heap {
    fn dereference(&self, reference: HeapReference) -> Option<ValueCell> {
        if let DroppableRegion::Alive(r) = self.raw.get(reference.generation)? {
            r.dereference(reference.reference)
        } else {
            None
        }
    }

    fn validate(&self, reference: &HeapReference) -> bool {
        if !self.is_alive(reference.generation) || reference.generation >= self.raw.len() {
            return false;
        }
        if let Some(DroppableRegion::Alive(region)) = self.raw.get(reference.generation) {
            region.validate(&reference.reference)
        } else {
            false
        }
    }
}

impl Debug for Heap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Heap [{:?}]", self.current)?;
        for (i, r) in self.raw.iter().enumerate() {
            writeln!(f, "  - Region #{} : {}", i, match r {
                DroppableRegion::Alive(region) => format!("Alive (&{})", region.len()),
                DroppableRegion::Dropped => "Dropped".to_string()
            })?;
        }
        Ok(())
    }
}
