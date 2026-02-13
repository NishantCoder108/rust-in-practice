### What is Mutex ?

> It has two rules for understanding.
>
> 1. Before access the data, we need acquire the lock.
> 2. When done with data, now we can unlock the data so other threads access the data.
>
> ---
>
> Locks the data -> Works with data -> If done with data -> unlock the data
>
> > Mutex care only one thread can access at a time. No other threads will be able to read or write.
