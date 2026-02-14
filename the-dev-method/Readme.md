## The Dev Method

> Personal Notes
>
> This page is my own collection of notes—written for myself as I learn and experiment. But if you’re reading this and want to understand these concepts, I’ve tried to make things as clear and beginner-friendly as possible!

---

### Reference Video

If you prefer learning by watching, check out this video:  
[The Dev Method (YouTube)](https://www.youtube.com/watch?v=dGjuAFxvXnU&list=PLAJ-sYO1aGdxQ_skPPtJ7PlSAjTXM-atv&index=27)

---

### Core Concept: Shared State

Rust provides a few building blocks for shared state:

- **Arc** — Share data across threads (atomic reference counting)
- **Rc** — Multiple owners within a single thread (non-atomic)
- **Mutex** — Ensure only one thread accesses data at a time

For a friendly, detailed guide,  
👉 [See my full notes on Arc, Rc, and Mutex here.](./00_shared-state/readme.md)
