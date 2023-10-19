Firefly is a fast pseudo random number generator intended for applications
including simulation, statistics, and randomized algorithms. It is not intended
for cryptographic applications.

The random number generator has a 128-bit state and outputs 64-bit numbers.
Its state space has size `2**128 - 1` (as it excludes zero) and its period is
the full space.

# Example Usage

```
use firefly128::Rng;
let mut g = Rng::from_u64(0);
let x = g.u64();
let mut h = g.split();
let y = g.u64();
```

# Algorithm

The random number generator has two parts.

-----------

A fast non-cryptographic random number generator.

The random number generator has a state space of size `2**128 - 1` and also
a period of length `2**128 - 1`.

# Design

## The State Space and Output Space

Like many other random number generator designs, this one can be viewed of
as a combination of two components: a state transition function and an
output function. Let `U` be the state space and `V` be the output space.
Then with the two functions

```text
f : U -> U
g : U -> V
```

the `i`th state and output are

```text
u_i = f(f(... f(u_0)))
      \_______/
       i times

v_i = g(u_i)
```

respectively. In our case, the state space is `NonZeroU128` and the
output space is `u64`.

```text
f : NonZeroU128 -> NonZeroU128
g : NonZeroU128 -> u64
```

The size of the state space was chosen because 64 bits is too small for
some plausible applications, while 128 bits should be sufficient for almost
all non-cryptographic purposes.

## The State Transition Function and its Period

The state transition function is a member of `GL(128, 2)`, that is, it is
an invertible linear transformation from the vector space of dimension 128
over the finite field of order 2 to itself.

In order to see that `f` is invertible, note that ...

TODO
