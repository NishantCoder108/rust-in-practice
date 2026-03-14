# Shared State with Mutex and Arc

## What is Mutex?

Mutex stands for "mutual exclusion" - it ensures that only one thread can access data at a time.

### Two Rules for Understanding Mutex:

1. **Before accessing the data, we need to acquire the lock.**
2. **When done with data, unlock it so other threads can access the data.**

---

**Simple Flow:**
```
Locks the data -> Works with data -> If done with data -> unlock the data
```

> **Important:** Mutex ensures only one thread can access at a time. No other threads will be able to read or write while the lock is held.

---

## Understanding Mutex - Step by Step

### 1. Using Mutex in Single-Threaded Context

```rust
let n = Mutex::new(9);

{
    let mut num = n.lock().unwrap();
    *num *= 12;
    // The lock is automatically released (unlocked) here, when 'num' goes out of scope.
} // <-- unlock happens here
```

**Understand with steps:**
1. First we acquire the lock: `n.lock()`
2. Now we can access the data and mutate it
3. Unlock happens automatically when the variable goes out of scope

**Note:** When printing a Mutex with Debug, you'll see fields like `data` and `poisoned`.
- `poisoned: false` means no panic occurred while the lock was held
- If a thread panics while holding the lock, the Mutex is considered "poisoned" to signal possible data inconsistency

---

### 2. Trying to Share Mutex with Multiple Threads (This Won't Work!)

```rust
let counter = Mutex::new(0);
let mut handles = vec![];

for _ in 0..10 {
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}
```

**Problem:** We can't move ownership in multiple threads!
- The value is moved from the first thread
- Other threads are still trying to access it
- This gives us a compilation error

**Solution:** We need multiple ownership with multiple threads!

---

### 3. Multiple Ownership with Multiple Threads (Using Arc + Mutex)

```rust
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

**How it works:**
- `Arc::clone()` doesn't clone the data, it just increments the reference counter
- Each thread gets its own `Arc` pointer to the same `Mutex`
- When we use `join()`, it waits for threads to finish their job sequentially

---

## Understanding Arc (Atomic Reference Counter)

### What is Arc?

**Arc** = Atomic Reference Counter pointer

**Atomic?** 
- Read, Add, Write operations happen in **one instruction**
- Other threads must wait - no partial operations can happen
- This makes it safe to use across multiple threads

---

## Rc vs Arc - The Key Difference

### Rc (Reference Counted) - Single Thread Only

```rust
let one_owner = Rc::new(String::from("Multiple Ownership"));
let second_owner = Rc::clone(&one_owner);
let third_owner = Rc::clone(&one_owner);
```

**Rc characteristics:**
- Use for **multiple ownership at single thread**
- Stores internally: `data` and `reference_counter`
- When we write `clone()`, it increases the reference counter
- When we `drop()`, it decreases the counter
- **Rc does NOT have `Send` trait** - so it can't cross threads
- Uses **non-atomic** reference counting

### Arc (Atomic Reference Counter) - Multiple Threads

**Arc characteristics:**
- Same as Rc but with **atomic** reference counting
- **Arc HAS `Send` trait** - so it can safely cross threads
- Use for **multiple ownership with multiple threads**

**Why Rc is not Send but Arc is?**
- Because Arc uses **atomic** reference counting
- Rc uses **non-atomic** reference counting
- Atomic operations are thread-safe, non-atomic are not

---

## Common Problems with Shared State

### Race Condition

**What is it?**
- Happens when we use something like `Rc` at multiple threads
- Suppose we have data:
  1. First thread is reading
  2. Another thread also reads the data before the first thread updates it
  3. Both threads add stuff
  4. Value should be 2, but it will be 1
  5. **That's a race condition!**

**Solution:** Use `Arc` + `Mutex` to prevent race conditions

---

### Deadlock

**What is it?**
- It's a logical error
- One thread is waiting for another thread
- The other thread is waiting for the first thread
- **They get stuck forever!**

**How to avoid:**
- Be careful about lock ordering
- Don't hold multiple locks unnecessarily
- Use timeouts when possible

---

## Summary

1. **Mutex** = Lock data → Work with data → Unlock data
2. **Rc** = Multiple ownership, single thread (non-atomic)
3. **Arc** = Multiple ownership, multiple threads (atomic)
4. **Arc + Mutex** = Safe shared state across threads
5. **Race Condition** = Multiple threads accessing data without proper synchronization
6. **Deadlock** = Threads waiting for each other forever
