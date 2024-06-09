pub mod memory;


#[cfg(test)]
mod tests {
    use roussillon_type_system::typing::concept::DataType;
    use roussillon_type_system::typing::primitive::Primitive;
    use roussillon_type_system::value::boolean::Boolean;
    use roussillon_type_system::value::byte::Bytes;
    use roussillon_type_system::value::concept::{DataValue, ValueCell};
    use roussillon_type_system::value::reference::Reference;
    use crate::memory::{Allocator, Dereference, Memory};

    fn test_type(r: &Reference, typename: &str) {
        println!("• {}", r.data_type().typename());
        assert_eq!(r.data_type().typename(), format!("&{}", typename));
    }

    fn test_cells(original_cell: &ValueCell, dereferenced: &ValueCell) {
        let t1 = original_cell.borrow().data_type().typename();
        let raw1 = original_cell.borrow().raw();
        let t2 = dereferenced.borrow().data_type().typename();
        let raw2 = dereferenced.borrow().raw();
        println!("{:?} <=> {:?}", original_cell.borrow(), dereferenced.borrow());
        assert_eq!(t1, t2);
        assert_eq!(raw1, raw2);
    }
    
    #[test]
    fn test_boolean_references() {
        let mut memory = Memory::default();
        
        let original_true = Boolean::create_true().to_cell();
        let original_false = Boolean::create_false().to_cell();
        let reference_true = memory.alloc_cell(original_true.clone());
        let reference_false = memory.alloc_cell(original_false.clone());
        test_type(&reference_true, &Primitive::Boolean.typename());
        test_type(&reference_false, &Primitive::Boolean.typename());
        let dereferenced_true = memory.dereference(reference_true).unwrap();
        let dereferenced_false = memory.dereference(reference_false).unwrap();
        test_cells(&original_true, &dereferenced_true);
        test_cells(&original_false, &dereferenced_false);
    }

    #[test]
    fn test_bytes_references() {
        let mut memory = Memory::default();

        for (data, t) in [
            (Bytes::Byte(77), Primitive::Byte.typename()),
            (Bytes::Arch(8855887455), Primitive::Bytes(8).typename()),
            (Bytes::Word(16584), Primitive::Bytes(2).typename()),
            (Bytes::Quad(998555), Primitive::Bytes(4).typename()),
            (Bytes::Long(8855887455), Primitive::Bytes(8).typename()),
            (Bytes::Wide(usize::MAX as u128 + 15550), Primitive::Bytes(16).typename()),
        ] {
            let original_cell = data.to_cell();
            let reference = memory.alloc_cell(original_cell.clone());
            test_type(&reference, &t);
            let dereferenced = memory.dereference(reference).unwrap();
            test_cells(&original_cell, &dereferenced);
        }
    }
}
