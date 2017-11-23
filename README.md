# typenum_loops
A library that provides loops which are fully or partially unrolled at compile time.



```rust
extern crate typenum;
extern crate typenum_loops;

use typenum::{U4, U6};
use typenum_loops::Loop;

fn main(){
    let arr: &mut[usize] = &mut[0; 4];
    // for i in 0..4 {arr[i] = i} fully unrolled by 4
    U4::full_unroll(&mut |i| arr[i] = i);
    
    let arr2: &mut[usize] = &mut[0; 13];
    // for i in 0..13 {arr2[i] = i} unrolled by 6
    U6::partial_unroll(13, &mut |i, _| arr2[i] = i);
}
```

Note: Very large closures may not be inlined despite attempts to trick llvm by using an empty wrapper fn.

## License
MIT