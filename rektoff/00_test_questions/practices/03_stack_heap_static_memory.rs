fn main() {
    let v = vec![11, 22];
    let l: &str = "ab";
    let s = String::from("cde"); // <-- HERE
}

/*

Stack (grows downwards):
+------------------+
| Function: main   |
+------------------+
| ret = [???]      |
+------------------+
| s.ptr = 0x1232   |
+------------------+
| [A]              | --> s.len = 3
+------------------+
| [B]              | --> s.cap = 3
+------------------+
| l.ptr = 0x0013   |
+------------------+
| [C]              | --> l.len = 2
+------------------+
| v.ptr = 0x1230   |
+------------------+
| [D]              | --> v.len = 2
+------------------+
| [E]              | --> v.cap = 2
+------------------+ <-- End of stack, start of heap
| 0x1234 = [F]     | --> 0x1234 = e
+------------------+
| 0x1233 = [G]     | --> 0x1233 = d
+------------------+
| 0x1232 = [H]     | --> 0x1232 = c
+------------------+
| 0x1231 = [I]     | --> 0x1231 = 22
+------------------+
| 0x1230 = [J]     | --> 0x1230 = 11
+------------------+ <-- Static Memory start
| 0x0014 = 'b'     |
+------------------+
| 0x0013 = 'a'     |
+------------------+
| 0x0012 = 'e'     |
+------------------+
| 0x0011 = 'd'     |
+------------------+
| 0x0010 = 'c'     |
+------------------+
*/

/*
-> vec and String will have ptr, len, cap
-> 'ab' , this will store inside static memory and It will have ptr and len


**--Interesting part--**
-> cde , It will also store inside static memory at the program starts.
-> After program start then its copy will store inside heap and it will mutate as well.
*/
