## Async Await Concepts

```rust

      fn main() {
          println!("Hello, world!");

          let x = foo1();
      }

      async fn foo1() -> usize {
          println!("foo");
          0
      }
```

> It will not print `foo`. why?
> Because of, If not waiting of future will return , so it means no need to call , if waiting. so it will not print.
