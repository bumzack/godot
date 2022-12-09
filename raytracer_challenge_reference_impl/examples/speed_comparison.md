# Speed comparison

cargo run --example  test_soft_shadow_aka_area_light --release

```
let size_factor = 3.0;
let antialiasing = true;
let antialiasing_size = 3; 

let usteps = 16;
let vsteps = 16;
```


## Apple M2

```
child thread ThreadId(2) finished. run for 9.307333ms , processed 52 lines
child thread ThreadId(3) finished. run for 35.233654291s , processed 52 lines
child thread ThreadId(4) finished. run for 35.450943625s , processed 54 lines
child thread ThreadId(5) finished. run for 35.450948541s , processed 54 lines
child thread ThreadId(6) finished. run for 35.450951583s , processed 54 lines
child thread ThreadId(7) finished. run for 35.45095425s , processed 53 lines
child thread ThreadId(8) finished. run for 35.526883833s , processed 55 lines
child thread ThreadId(9) finished. run for 35.596672625s , processed 53 lines
child thread ThreadId(10) finished. run for 35.596676916s , processed 53 lines
multi core duration: 35.59668325s with AA size = 3
multi core duration: 35.599768333s with AA size = 3
```


```
let size_factor = 3.0;
let antialiasing = true;
let antialiasing_size = 3; 

let usteps = 8;
let vsteps = 8;
```


```
child thread ThreadId(2) finished. run for 7.925333ms , processed 52 lines
child thread ThreadId(3) finished. run for 10.078404541s , processed 53 lines
child thread ThreadId(4) finished. run for 10.108555791s , processed 53 lines
child thread ThreadId(5) finished. run for 10.130502375s , processed 52 lines
child thread ThreadId(6) finished. run for 10.174888291s , processed 54 lines
child thread ThreadId(7) finished. run for 10.177182791s , processed 54 lines
child thread ThreadId(8) finished. run for 10.17718725s , processed 54 lines
child thread ThreadId(9) finished. run for 10.177191583s , processed 54 lines
child thread ThreadId(10) finished. run for 10.177195541s , processed 54 lines
multi core duration: 10.17720125s with AA size = 3
multi core duration: 10.180691833s with AA size = 3
```


