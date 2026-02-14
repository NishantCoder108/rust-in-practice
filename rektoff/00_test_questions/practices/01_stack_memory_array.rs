fn main() {
    let a: i32 = 10;
    let b: [i32; 3] = [1, 2, 3]; // <-- HERE

    println!("a pointer -> {:p}", &a);
    println!("b pointer -> {:p}", &b);

    /*This work as well */
    let p: *const i32 = &a;
    println!("p pointer -> {:p}", p);

    let b0 = &b[0] as *const i32;
    println!("b[0] pointer -> {:p}", b0);

    let b1 = &b[1] as *const i32;
    println!("b[1] pointer -> {:p}", b1);

    let b2 = &b[2] as *const i32;
    println!("b[2] pointer -> {:p}", b2); 

    /*
    **------Output------**

    a pointer -> 0x16b88de68
    b pointer -> 0x16b88de6c
    p pointer -> 0x16b88de68
    b[0] pointer -> 0x16b88de6c
    b[1] pointer -> 0x16b88de70
    b[2] pointer -> 0x16b88de74


    **---Final Conclusion---**
    -> Pointer of b is pointed to the b0 address. And it will be exact same.
    */
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


Output:
Stack (grows downwards):
+------------------+
| Function: main   |
+------------------+
| ret = [???]      |
+------------------+
| b[2] = 3         |
+------------------+
| b[1] = 2         |
+------------------+
| b[0] = 1         |
+------------------+
| a = 10           |
+------------------+

*/

/*
-> At data frame , data frame goes upto top. eg. v = [1,2,3]

***---Data frame---***
v[2] = 3
v[1] = 2
v[0] = 1


-> It will go higher address to lower address
-> Lower address is near to memory. like instruction data or text segment data is near to 0

(high addresses)
+------------------+
| 0x9000           |
+------------------+
| 0x8000           |
+------------------+
| 0x7000           |
+------------------+
(low addresses)


-> Each new stack frame is allocated below the previous frame (in test diagram) -> goes downward
-> And Each stack frame data will goes to top


***---Frame layout:---***
-> In frame, last variable will show in top in frame. and first variable will show at the bottom of frame.
-> return (ret) slot top of frame
-> argument will show at the bottom of frame.
-> arguments or function parameter will show in frame like - first come will go bottom of frame.


Looks like:
_____________

(high)
| ret = 10 |
| locals   |
| args     |
(low)

-> contagious data : first or nearer goes to bottom of frame and remaining next to each other

(high)
| t.1 = 'x' |
| t.0 = 10  |
(low)

or

(high)
| (2,'b').1 |
| (2,'b').0 |
| (1,'a').1 |
| (1,'a').0 |
(low)
*/
