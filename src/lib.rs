#![doc = include_str!("../README.md")]
#![no_std]

use core::num::NonZeroU128;

#[inline(always)]
const fn mul(x: u64, y: u64) -> u128 {
  (x as u128) * (y as u128)
}

#[inline(always)]
const fn lo(x: u128) -> u64 {
  x as u64
}

#[inline(always)]
const fn hi(x: u128) -> u64 {
  (x >> 64) as u64
}

#[inline(always)]
const fn concat(x: u64, y: u64) -> u128 {
  (x as u128) ^ ((y as u128) << 64)
}

/// A fast non-cryptographic random number generator.

#[derive(Clone, PartialEq, Eq)]
pub struct Rng(NonZeroU128);

impl Rng {
  /// Creates a new random number generator with the given initial state. A
  /// good state should be drawn from a distribution with sufficient entropy.

  #[inline(always)]
  pub const fn new(state: NonZeroU128) -> Self {
    Self(state)
  }

  /// Retrieves the random number generator's current state.

  #[inline(always)]
  pub const fn state(&self) -> NonZeroU128 {
    self.0
  }

  /// Creates a new random number generator using the given seed to create the
  /// initial state. A good seed should be drawn from a distribution with
  /// sufficient entropy.

  pub const fn from_seed(seed: [u8; 16]) -> Self {
    let s = u128::from_le_bytes(seed);
    let s = s ^ (s == 0) as u128;
    let s = unsafe { NonZeroU128::new_unchecked(s) };
    Self(s)
  }

  /// Creates a new random number generator by hashing the given integer to
  /// produce the initial state.

  pub const fn from_u64(n: u64) -> Self {
    const M: u128 = 0x487e_d511_0b46_11a6_2633_145c_06e0_e689;
    let s = concat(n, 1);
    let s = s.wrapping_mul(M);
    let s = s.swap_bytes();
    let s = s.wrapping_mul(M);
    let s = s.swap_bytes();
    let s = s.wrapping_mul(M);
    let s = unsafe { NonZeroU128::new_unchecked(s) };
    Self(s)
  }

  /// Samples a `u64` from the uniform distribution.

  #[inline(always)]
  pub fn u64(&mut self) -> u64 {
    let s = self.0.get();
    let a = lo(s);
    let b = hi(s);
    let c = a.rotate_right(7) ^ b;
    let d = a ^ a >> 19;
    let t = mul(b, b);
    let x = a ^ lo(t).wrapping_add(hi(t));
    let s = concat(c, d);
    let s = unsafe { NonZeroU128::new_unchecked(s) };
    self.0 = s;
    x
  }

  /// Splits off a new random number generator that may be used along with the
  /// original.

  #[inline(always)]
  pub fn split(&mut self) -> Self {
    let a = self.u64();
    let b = self.u64();
    let s = concat(a, b);
    let s = s ^ (s == 0) as u128;
    let s = unsafe { NonZeroU128::new_unchecked(s) };
    Self(s)
  }

  /// Fills a slice with i.i.d. bytes sampled from the uniform distribution.

  pub fn fill_u8(&mut self, dst: &mut [u8]) {
    if dst.len() == 0 { return; }

    let mut dst = dst;
    let mut x;
    let mut y;

    // TODO: for long enough slices, we can do overlapping writes for the end.

    loop {
      x = self.u64();
      y = self.u64();
      if dst.len() < 16 { break; }
      dst[0 ..= 7].copy_from_slice(&x.to_le_bytes());
      dst[8 ..= 15].copy_from_slice(&y.to_le_bytes());
      dst = &mut dst[16 ..];
    }

    if dst.len() >= 8 {
      dst[0 ..= 7].copy_from_slice(&x.to_le_bytes());
      dst = &mut dst[8 ..];
    }

    while dst.len() >= 1 {
      dst[0] = y as u8;
      y >>= 8;
      dst = &mut dst[1 ..];
    }
  }
}

#[cfg(feature = "rand_core")]
impl rand_core::RngCore for Rng {
  #[inline(always)]
  fn next_u32(&mut self) -> u32 {
    self.u64() as u32
  }

  #[inline(always)]
  fn next_u64(&mut self) -> u64 {
    self.u64()
  }

  fn fill_bytes(&mut self, dest: &mut [u8]) {
    self.fill_u8(dest)
  }

  fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
    self.fill_u8(dest);
    Ok(())
  }
}

#[cfg(feature = "rand_core")]
impl rand_core::SeedableRng for Rng {
  type Seed = [u8; 16];

  fn from_seed(seed: Self::Seed) -> Self {
    Self::from_seed(seed)
  }

  fn seed_from_u64(state: u64) -> Self {
    Self::from_u64(state)
  }
}
