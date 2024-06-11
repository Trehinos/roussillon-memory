use roussillon_type_system::value::concept::ValueCell;
use roussillon_type_system::value::reference::Reference;
use crate::region::{Allocator, Dereference, Region};

#[derive(Clone)]
pub enum RegionValidity {
    Alive(Region),
    Dropped,
}

impl RegionValidity {
    pub fn is_alive(&self) -> bool {
        matches!(self, Self::Alive(_))
    }

    pub fn is_dropped(&self) -> bool {
        matches!(self, Self::Dropped)
    }

    pub fn unwrap(&self) -> &Region {
        match self {
            RegionValidity::Alive(region) => region,
            RegionValidity::Dropped => panic!("Attempted to unwrap a dropped region"),
        }
    }
}

pub struct HeapReference {
    generation: usize,
    reference: Reference,
}

impl HeapReference {
    pub fn reference(&self) -> &Reference {
        &self.reference
    }
}

#[derive(Default)]
pub struct Heap {
    raw: Vec<RegionValidity>,
    current: Option<usize>,
}

impl Heap {
    pub fn new() -> Self { Heap { raw: Vec::new(), current: None } }
    pub fn current_generation(&self) -> Option<usize> { self.current }
    pub fn next_generation(&mut self) -> &Region {
        self.current = match self.current {
            None => Some(0),
            Some(s) => Some(s + 1),
        };
        self.raw.push(RegionValidity::Alive(Region::default()));
        self.raw.last().unwrap().unwrap()
    }
    pub fn clear(&mut self, generation: usize) { self.raw[generation] = RegionValidity::Dropped; }
    pub fn is_alive(&self, generation: usize) -> bool { self.raw[generation].is_alive() }
}

impl Allocator<HeapReference> for Heap {
    fn allocate(&mut self, cell: ValueCell) -> HeapReference {
        let current = self.current_generation().unwrap();
        if let RegionValidity::Alive(region) = self.raw.get_mut(current).unwrap() {
            let r = region.allocate(cell);
            HeapReference { generation: self.current.unwrap(), reference: r }
        } else {
            panic!("Invalid region in heap.")
        }
    }
}

impl Dereference<HeapReference> for Heap {
    fn dereference(&self, reference: HeapReference) -> Option<ValueCell> {
        if let RegionValidity::Alive(r) = self.raw.get(reference.generation)? {
            r.dereference(reference.reference)
        } else {
            None
        }
    }

    fn validate(&self, reference: &HeapReference) -> bool {
        if !self.is_alive(reference.generation) || reference.generation >= self.raw.len() {
            return false;
        }
        if let Some(RegionValidity::Alive(region)) = self.raw.get(reference.generation) {
            region.validate(&reference.reference)
        } else {
            false
        }
    }
}