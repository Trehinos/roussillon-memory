pub mod region;
#[cfg(feature = "heap")]
pub mod heap;
#[cfg(feature = "stack")]
pub mod stack;

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use roussillon_type_system::{
        types::{
            concept::DataType,
            primitive::Primitive,
        },
        value::{
            concept::{DataValue, ValueCell},
            boolean::Boolean,
            byte::Bytes,
            number::{Float, Integer},
            record::Record,
            reference::Reference,
        },
    };
    use roussillon_type_system::factory::create_struct;
    use roussillon_type_system::identity::LabelBank;
    use roussillon_type_system::types::typedef::Structure;

    use crate::heap::Heap;
    use crate::region::{Allocator, Dereference, Region};

    fn test_type(r: &Reference, typename: &str) {
        // println!("• {}", r.data_type().typename());
        assert_eq!(r.data_type().typename(), format!("&{}", typename));
    }

    fn test_cells(original_cell: &ValueCell, dereferenced: &ValueCell) {
        let t1 = original_cell.borrow().data_type().typename();
        let raw1 = original_cell.borrow().raw();
        let t2 = dereferenced.borrow().data_type().typename();
        let raw2 = dereferenced.borrow().raw();
        // println!("{:?} <=> {:?}", original_cell.borrow(), dereferenced.borrow());
        assert_eq!(t1, t2);
        assert_eq!(raw1, raw2);
    }

    fn test_struct() -> Rc<Structure> {
        create_struct("MyStruct", LabelBank::from(&[
            "field_a",
            "field_b",
            "field_c",
        ]), &[
            Primitive::Integer.to_rc(),
            Primitive::Integer.to_rc(),
            Primitive::Float.to_rc(),
        ])
    }

    fn test_object(structure: Rc<Structure>) -> ValueCell {
        Record::new(structure, &[
            Integer::new(40).to_cell(),
            Integer::new(96).to_cell(),
            Float::new(40.0).to_cell()
        ]).unwrap().to_cell()
    }

    #[test]
    fn test_struct_reference() {
        let mut memory = Region::default();
        let my_struct = test_struct();
        // println!("\n{:?}", my_struct);

        let original_object = test_object(my_struct.clone());

        let reference = memory.allocate(original_object.clone());
        test_type(&reference, "MyStruct");
        let dereferenced_object = memory.dereference(reference).unwrap();
        test_cells(&original_object, &dereferenced_object);
    }

    #[test]
    fn test_heap() {
        let mut heap = Heap::new();
        // println!("{:?}", heap);
        assert_eq!(heap.current_generation(), None);


        heap.next_generation();
        // println!("{:?}", heap);
        assert_eq!(heap.current_generation(), Some(0));

        let my_struct = test_struct();
        // println!("\n{:?}", my_struct);

        let original_object = test_object(my_struct.clone());

        let reference = heap.allocate(original_object.clone());
        println!("{:?}", reference);

        test_type(reference.reference(), "MyStruct");
        let dereferenced_object = heap.dereference(reference.clone()).unwrap();
        test_cells(&original_object, &dereferenced_object);
        // println!("{:?}", heap);

        heap.next_generation();
        assert_eq!(heap.current_generation().unwrap(), 1);
        // println!("{:?}", heap);

        heap.clear(0);
        // println!("{:?}", heap);
        assert!(!heap.validate(&reference));
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

        let original_cell = Integer::new(1415).to_cell();
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
