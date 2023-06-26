Firefly is a fast pseudo random number generator intended for applications
including simulation, statistics, and randomized algorithms. It is not intended
for cryptographic applications.

The random number generator has a 128-bit state and outputs 64-bit numbers.
Its state space has size `2**128 - 1` (as it excludes zero) and its period is
the full space.

# Example Usage

```
use firefly_rng::Rng;
let mut g = Rng::from_u64(0);
let x = g.u64();
let mut h = g.split();
let y = g.u64();
```

# Algorithm

The random number generator has two parts.
