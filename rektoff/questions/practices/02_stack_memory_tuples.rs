fn main() {
    let tup: (i32, usize) = (1, 2);
    let arr = [3, 4]; // <-- HERE

    println!("arr pointer -> {:p}", &arr);
    println!("tup pointer -> {:p}", &tup);

    let arr1 = &arr[1] as *const i32;
    println!("arr[1] pointer -> {:p}", arr1);

    let arr0 = &arr[0] as *const i32;
    println!("arr[0] pointer -> {:p}", arr0);
    println!("arr[0] value -> {}", arr[0]);

    let tup1 = &tup.1 as *const _;
    println!("tup.1 pointer -> {:p}", tup1);

    let tup0 = &tup.0 as *const _;
    println!("tup.0 pointer -> {:p}", tup0);
}

/*
Stack (grows downwards):
+------------------+
| Function: main   |
+------------------+
| ret = [???]      |
+------------------+
| [A]              |
+------------------+
| [B]              |
+------------------+
| [C]              |
+------------------+
| [D]              |
+------------------+


***----Result----***

Stack (grows downwards):
+------------------+
| Function: main   |
+------------------+
| ret = [???]      |
+------------------+
| arr[1] = 4       |
+------------------+
| arr[0] = 3       |
+------------------+
| tup.1 = 2        |
+------------------+
| tup.0 = 1        |
+------------------+



***---Prints---***
arr pointer -> 0x16b221e10
tup pointer -> 0x16b221e00
arr[1] pointer -> 0x16b221e14
arr[0] pointer -> 0x16b221e10
arr[0] value -> 3
tup.1 pointer -> 0x16b221e08
tup.0 pointer -> 0x16b221e00
*/
