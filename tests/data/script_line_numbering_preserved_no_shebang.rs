/*!
This is merged into a default manifest in order to form the full package manifest:

```cargo
[dependencies]
boolinator = "=0.1.0"
```
 */
use boolinator::Boolinator;
fn main() {
    println!("line: {}",line!());
}