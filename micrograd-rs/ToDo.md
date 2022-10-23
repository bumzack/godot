# ToDos

- duplicate nodes in graph for ```let c = &a + &a```

- why is ```impl<T> Display for ValueRefV1<T>``` not working for a f64. resp why is the generic impl not working. guess this should be doable
- why is the order in the topo vec different from the https://youtu.be/VMj-3S1tku0?t=4797
the b is not in the correct order


## remember
don't implement ```let c = a+ b``` which moves the values. cloning does not help, because then 
we create a new node/value which we dont want
