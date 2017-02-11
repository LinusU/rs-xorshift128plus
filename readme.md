# XorShift128Plus

Rust implementation of the psuedo random number generator  [xorshift128+](http://xoroshiro.di.unimi.it).

[📔 Documentation](http://docs.rs/xorshift128plus)

## Usage

```rust
extern crate xorshift128plus;

use xorshift128plus::XorShift128Plus;

fn main() {
  let mut rng = XorShift128Plus::from_u32(4293262078);

  println!("First random float: {}", rng.next());
  println!("Second random float: {}", rng.next());
}
```
