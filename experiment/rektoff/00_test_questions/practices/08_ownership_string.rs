/*
Here is some code, and some Lexical Lifetimes (LL) notation, but it has an error. Answer the following:

A. Provide the line number where the analysis first goes wrong (what line of notation).

NOTE: Make sure to verify all lines above your answer are correct as you are reporting the earliest error.

B. For Lexical Lifetime (LL), what should the first element of the main set of line 8 be? i.e. main{B, ...} what is B?
C. For Lexical Lifetime (LL), what should the second element of the main set of line 8 be? i.e. main{..., C} what is C?
D. For Non-Lexical Lifetime (NLL), what should the first element of the main set of line 12 be? i.e. main{D, ...} what is D?
E. For Non-Lexical Lifetime (NLL), what should the second element of the main set of line 12 be? i.e. main{..., E} what is E?
If an answer should be empty, you must acknowledge that it is empty by writing "Empty" - leaving the field empty is not the same as acknowledging emptiness.

*/
fn main() {
    let mut s = String::from("hello");

    let r = &s;
    println!("{}", r);

    s.push_str(" world");
    println!("{}", s);
}

/*
1 fn main() {
2     // main{}
3     let mut s = String::from("hello");
4     // main{mut s}
5     let r = &s;
6     // main{mut s, r(&s)}
7     println!("{}", r); // main{mut s, r(&s)} ✅ r is in ownership set for main
8     // main{mut s, r(&s)}
9
10     s.push_str(" world"); // main{mut s, r(&s), push_str{self(&mut s)}}
11     // main{mut s, r(&s)}
12     println!("{}", s); // main{mut s, r(&s)} ✅ s is in ownership set for main
13     // main{mut s, r(&s)}
14 } // nothing - main ends, s and r are dropped by LL


****---Result---****
A -> 6
B -> shrs s
C -> r(&s)
D -> mut s
E -> Empty
*/

/* Rapid Questions */

/*


Section 1. Rapid Questions
1. How does Rust handle ownership of (non-static) variables in the calling when another function is called?
1. How does Rust handle ownership of (non-static) variables in the calling when another function is called?
A

Functions can access variables from the calling scope directly.
B

Functions automatically borrow variables from the calling scope
C

Functions receive ownership or borrow variables explicitly through parameters
D

Functions share ownership of all variables with the calling scope
2. What is the primary advantage of Non-Lexical Lifetimes (NLL) over Lexical Lifetimes (LL) in Rust?
2. What is the primary advantage of Non-Lexical Lifetimes (NLL) over Lexical Lifetimes (LL) in Rust?
A

NLL allows variables to live longer than their scopes
B

NLL enables the borrow checker to consider the actual usage of references, not just their textual scope
C

NLL removes the need for the borrow checker
D

NLL allows multiple mutable references simultaneously
3. Which types in Rust automatically implement the Copy trait?
*
3. Which types in Rust automatically implement the Copy trait?
A

All types that implement the Clone trait.
B

All types that implement the Sized trait.
C

Primitive scalar types like integers, floats, and booleans.
D

All heap-allocated types.
4. In safe Rust, if you have a mutable reference (&mut T) to some data, what can you conclude about all other code during the lifetime of that borrow?
*
4. In safe Rust, if you have a mutable reference (&amp;mut T) to some data, what can you conclude about all other code during the lifetime of that borrow?
A

Other code may still read the data but not modify it.
B

Other code may read or write the data, but only from the same thread.
C

No other code in any thread has read or write access to that data
D

The original owner retains read access.
5. What is the effect of passing a Copy type variable to a function in Rust?
*
5. What is the effect of passing a Copy type variable to a function in Rust?
A

The function takes ownership, and the original variable becomes invalid.
B

The function borrows the variable, and the original variable remains valid.
C

The variable is duplicated, and both the original and the copy remain valid.
D

The variable is moved, and the original variable becomes invalid.
6. What happens when a non-Copy type variable is assigned to another variable in Rust?
*
6. What happens when a non-Copy type variable is assigned to another variable in Rust?
A

Both variables share ownership of the data.
B

The data is duplicated, and both variables own separate copies.
C

Ownership of the data the variable owns is transferred to the new variable.
D

The compiler throws an error due to ownership conflict.
7. If a program reads an index from user input and uses it to access an array element, how does the Rust compiler handle this?
*
7. If a program reads an index from user input and uses it to access an array element, how does the Rust compiler handle this?
A

It does nothing, since all bounds checks must happen at compile time.
B

It inserts a runtime bounds check that will panic if the index is out of bounds.
C

It emits a compile-time warning that the index might be invalid.
D

It produces a compile-time error because the index value is not statically known.
8. Why can't you use an expression like t.(1 + 1) to access elements in a tuple in Rust?
*
8. Why can't you use an expression like t.(1 + 1) to access elements in a tuple in Rust?
A

Because tuple indices must be compile-time constants.
B

Because tuple indices are treated as field names, not as computed expressions.
C

Because Rust does not support tuple indexing.
D

Because tuple elements can only be accessed through pattern matching.
9. What is a "fat pointer" in Rust?
*
9. What is a "fat pointer" in Rust?
A

A pointer that includes type information for dynamic dispatch.
B

A pointer that points to larger items in memory.
C

A pointer that includes both a memory address and metadata like length.
D

A pointer that can point to multiple types of data.
10. How does Rust prevent buffer overflows in safe code?
10. How does Rust prevent buffer overflows in safe code?
A

By using garbage collection to manage memory.
B

By allowing unchecked memory access for performance.
C

By enforcing ownership and borrowing rules at runtime.
D

By performing compile-time and runtime bounds checks.

*/
