pub mod memory;


#[cfg(test)]
mod tests {
    use roussillon_type_system::types::algebraic::ProductType;
    use roussillon_type_system::types::concept::DataType;
    use roussillon_type_system::types::primitive::Primitive;
    use roussillon_type_system::types::typedef::Structure;
    use roussillon_type_system::value::boolean::Boolean;
    use roussillon_type_system::value::byte::Bytes;
    use roussillon_type_system::value::concept::{DataValue, ValueCell};
    use roussillon_type_system::value::number::{Float, Integer};
    use roussillon_type_system::value::record::Record;
    use roussillon_type_system::value::reference::Reference;
    use crate::memory::{Allocator, Dereference, Region};

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
    fn test_struct_reference() {
        let mut memory = Region::default();

        let my_struct = Structure::new("MyStruct", ProductType::new(&[
            Primitive::Integer.to_rc(),
            Primitive::Integer.to_rc(),
            Primitive::Float.to_rc(),
        ])).to_rc();
        println!("\n{:?}", my_struct);

        let original_object = Record::new(my_struct.clone(), &[
            Integer::new(40).to_cell(),
            Integer::new(96).to_cell(),
            Float::new(40.0).to_cell()
        ]).unwrap().to_cell();
        
        let reference = memory.allocate(original_object.clone());
        test_type(&reference, "MyStruct");
        let dereferenced_object = memory.dereference(reference).unwrap();
        test_cells(&original_object, &dereferenced_object);
        
        
    }
    
    #[test]
    fn test_boolean_references() {
        let mut memory = Region::default();
        
        let original_true = Boolean::create_true().to_cell();
        let original_false = Boolean::create_false().to_cell();
        let reference_true = memory.allocate(original_true.clone());
        let reference_false = memory.allocate(original_false.clone());
        test_type(&reference_true, &Primitive::Boolean.typename());
        test_type(&reference_false, &Primitive::Boolean.typename());
        let dereferenced_true = memory.dereference(reference_true).unwrap();
        let dereferenced_false = memory.dereference(reference_false).unwrap();
        test_cells(&original_true, &dereferenced_true);
        test_cells(&original_false, &dereferenced_false);
    }

    #[test]
    fn test_float_reference() {
        let mut memory = Region::default();

        let original_cell = Float::new(std::f64::consts::PI).to_cell();
        let reference = memory.allocate(original_cell.clone());
        test_type(&reference, &Primitive::Float.typename());
        let dereferenced = memory.dereference(reference).unwrap();
        test_cells(&original_cell, &dereferenced);
    }

    #[test]
    fn test_integer_reference() {
        let mut memory = Region::default();

        let original_cell = Integer::new(-1415).to_cell();
        let reference = memory.allocate(original_cell.clone());
        test_type(&reference, &Primitive::Integer.typename());
        let dereferenced = memory.dereference(reference).unwrap();
        test_cells(&original_cell, &dereferenced);
    }

    #[test]
    fn test_bytes_references() {
        let mut memory = Region::default();

        for (data, t) in [
            (Bytes::Byte(77), Primitive::Byte.typename()),
            (Bytes::Arch(16584), Primitive::Bytes(std::mem::size_of::<usize>()).typename()),
            (Bytes::Word(16584), Primitive::Bytes(2).typename()),
            (Bytes::Quad(998555), Primitive::Bytes(4).typename()),
            (Bytes::Long(8855887455), Primitive::Bytes(8).typename()),
            (Bytes::Wide(usize::MAX as u128 + 15550), Primitive::Bytes(16).typename()),
        ] {
            let original_cell = data.to_cell();
            let reference = memory.allocate(original_cell.clone());
            test_type(&reference, &t);
            let dereferenced = memory.dereference(reference).unwrap();
            test_cells(&original_cell, &dereferenced);
        }
    }
}
