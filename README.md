# Roussillon : Memory

This crate provides some structs and trait to manage memory and references for an abstract language.

## Traits

* `Allocator`
* `Dereference`

## Structures

* `Region` : a vector containing `ValueCell`s,
  * `Area` : a catalog of named `Regions`,
  * `DroppableRegion` : a `Region` which can be `active` ou `dropped`.
* `Heap` : a memory structure (generation-arena like) with :
  * `HeapReference` : a `DataValue` which contains a `Reference` to a `Region`'s generation in a `Heap`.
  * `HeapReferenceType` : the `DataType` of a `HeapReference` value.
* `Stack` : a FIFO memory structure with :
  * `StackReference` : a `DataValue` which contains a `Reference` to a `Region` in a `Stack`.
  * `StackReferenceType` : the `DataType` of a `StackReference` value.

## License

(c) 2024 Sébastien Geldreich

This work is published under the MIT License.