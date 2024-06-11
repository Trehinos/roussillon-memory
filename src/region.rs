use std::collections::HashMap;
use roussillon_type_system::identity::Label;
use roussillon_type_system::value::reference::Reference;
use roussillon_type_system::value::concept::{DataValue, ValueCell};

pub trait Allocator<R=Reference, C=ValueCell> {
    fn allocate(&mut self, cell: C) -> R;
}

pub trait Dereference<R=Reference, C=ValueCell> {
    fn dereference(&self, reference: R) -> Option<C>;
    fn validate(&self, reference: &R) -> bool;
}

#[derive(Clone, Default, Debug)]
pub struct Region {
    raw: Vec<u8>,
}

impl Region {
    pub fn len(&self) -> usize { self.raw.len() }
    pub fn is_empty(&self) -> bool { self.raw.is_empty() }
}

impl Allocator for Region {
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

impl Dereference for Region {
    fn dereference(&self, reference: Reference) -> Option<ValueCell> {
        let start = reference.get_address();
        let referenced_type = reference.referenced();
        let end = start + referenced_type.size();
        if end > self.raw.len() { return None; };
        let raw = &self.raw[start..end];
        Some(referenced_type.construct_from_raw(raw).unwrap())
    }

    fn validate(&self, reference: &Reference) -> bool {
        (reference.get_address() + reference.data_type().size()) < self.len()
    }
}

pub struct Area {
    regions: HashMap<String, Region>,
}

impl Area {
    pub fn empty() -> Self { Self { regions: HashMap::new() } }

    pub fn get(&self, label: &Label) -> Option<&Region> {
        self.regions.get(&label.to_string())
    }
    
    pub fn take(&mut self, label: &Label) -> Option<Region> {
        self.regions.remove(&label.to_string())
    }
    
    pub fn set(&mut self, label: &Label, region: Region) -> Option<Region> {
        self.regions.insert(label.to_string(), region)
    }
}