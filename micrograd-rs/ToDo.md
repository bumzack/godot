# ToDos

- duplicate nodes in graph for ```let c = &a + &a```
- why is the original code using a Set for the children and it still works with the vec in the rust impl?
  does this need to be changed. tests seem fine so far.
- why is ```impl<T> Display for ValueRefV1<T>``` not working for a f64. resp why is the generic impl not working. guess
  this should be doable
- why is the order in the topo vec different from the https://youtu.be/VMj-3S1tku0?t=4797
  the b is not in the correct order
- moondata net takes 400 - 450 epochs to reach accuracy of 100 %, in the original post 100 epochs are sufficient (dataset is probably quiet different, as it is not the original scikit-learn method to generate the training data)
- moondata net does not always converge to minimum - sometimes the accuracy stays at 50%

## remember

don't implement ```let c = a+ b``` which moves the values. cloning does not help, because then
we create a new node/value which we dont want
