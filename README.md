# Roussillon : Memory

This crate provides some structs and trait to manage memory and references for an abstract language.

## Traits

* `Allocator`
* `Dereference`

## Structures

* `Region` : a vector containing `ValueCell`s,
* `Heap` : a memory structure (generation-arena like) with `HeapReference`s.
* `Stack` : a FIFO memory structure with `StackReference`s.

## License

(c) 2024 Sébastien Geldreich

This work is published under the MIT License.