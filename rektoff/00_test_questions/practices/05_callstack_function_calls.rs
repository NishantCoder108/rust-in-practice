/*Program 1 */
fn main() {
    let a = f();
}
fn f() {
    g()
}
fn g() {
    // Do nothing
}

/*
**---Program2---**

fn main() {
    let a = f();
    g();
}
fn f() {
    // Do nothing
}
fn g() {
    // Do nothing
}




 ****-----Print this things-----****


 Stack (grows downwards):
+------------------+
| Function: main   |
+------------------+
| a = [???]        |
+------------------+
| Function: f      |
+------------------+
| ret = ()         |
+------------------+
| Function: g      |
+------------------+
| ret = ()         |
+------------------+


*****---Conclusion---*****
-> variable a assigns. It will invoke function f and f will invoke g so every function create
-> scope. Noone will goes out of scope



Result - 1

*/
