fn main() {}

/*

Answer 'Y' or 'N' in each box below. Blank answers are marked incorrect.

First, read the 3 rules for Rust's default layout (#[repr(Rust)]) in the Rust Reference and consider how they apply to structs.

Below is a diagram of a Vec's memory layout from the Rust source code. Based on the layout rules, which orderings of ptr, len, and capacity are possible on the stack?

Y = ordering is possible
N = ordering is not possible

Note: Assume PhantomData and Allocator are zero-sized and can be ignored.

///             ptr      len      cap
///        +--------+--------+--------+
///        | 0x0123 |      2 |      4 |
///        +--------+--------+--------+
///             |
///             v
/// Heap   +--------+--------+--------+--------+
///        |    'a' |    'b' | uninit | uninit |
///        +--------+--------+--------+--------+
///


****---Ref---****
https://doc.rust-lang.org/reference/type-layout.html#r-layout.repr.rust.layout

diagram: https://github.com/rust-lang/rust/blob/c40c51f9fdfa90b9c91c1601ec1442225c6b5c36/library/alloc/src/vec/mod.rs#L343-L351




Conclusions:
-> `repr(C)` -> If we use this one, so it gives guarntee the ordering
-> `repr(Rust)` -> It does not give guarntee, it just give minimum guarntee, not ordering




----Some Points-----
1. The only data layout guarantees made by this representation are those required for soundness. They are:

The fields are properly aligned.
The fields do not overlap.
The alignment of the type is at least the maximum alignment of its fields.



Result:

(ptr, len, cap): Y
(ptr, cap, len): Y
(len, ptr, cap): Y
(len, cap, ptr): Y
(cap, ptr, len): Y
(cap, len, ptr): Y

*/
