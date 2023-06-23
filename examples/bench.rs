use std::time::Instant;
use std::hint;
use firefly_rand::Rng as Firefly;

const COUNT: usize = 100_000_000;

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

pub trait BenchRng {
  fn from_seed(seed: [u8; 16]) -> Self;

  fn u64(&mut self) -> u64;

  #[inline(never)]
  fn u64_noinline(&mut self) -> u64 { self.u64() }
}

impl BenchRng for Firefly {
  fn from_seed(seed: [u8; 16]) -> Self {
    Self::from_seed(seed)
  }

  #[inline(always)]
  fn u64(&mut self) -> u64 {
    self.next()
  }
}

struct Xoroshiro128pp {
  x: u64,
  y: u64,
}

impl BenchRng for Xoroshiro128pp {
  fn from_seed(seed: [u8; 16]) -> Self {
    let s = u128::from_le_bytes(seed);
    let s = s ^ (s == 0) as u128;
    Self { x: lo(s), y: hi(s) }
  }

  #[inline(always)]
  fn u64(&mut self) -> u64 {
    let x = self.x;
    let y = self.y;
    let z = x.wrapping_add(y).rotate_left(17).wrapping_add(x);
    let y = x ^ y;
    let x = x.rotate_left(49) ^ y ^ y << 21;
    let y = y.rotate_left(28);
    self.x = x;
    self.y = y;
    z
  }
}

struct Pcg64dxsm {
  x: u128,
}

impl BenchRng for Pcg64dxsm {
  fn from_seed(seed: [u8; 16]) -> Self {
    Self { x: u128::from_le_bytes(seed) }
  }

  #[inline(always)]
  fn u64(&mut self) -> u64 {
    let x = self.x;
    let a = lo(x);
    let b = hi(x);
    let a = a | 1;
    let b = b ^ b >> 32;
    let b = b * 0xda942042e4dd58b5;
    let b = b ^ b >> 48;
    let b = b * a;
    let x = 0xda942042e4dd58b5 * x + 1;
    self.x = x;
    b
  }
}

struct Mwc256xxa64 {
  x: u64,
  y: u64,
  z: u64,
  c: u64,
}

impl BenchRng for Mwc256xxa64 {
  fn from_seed(seed: [u8; 16]) -> Self {
    let seed = u128::from_le_bytes(seed);
    let x = lo(seed);
    let y = hi(seed);
    Self {
      x,
      y,
      z: 0xcafef00dd15ea5e5,
      c: 0x14057b7ef767814f,
    }
  }

  fn u64(&mut self) -> u64 {
    const M: u64 = 0xfeb3_4465_7c0a_f413;
    let x = self.x;
    let y = self.y;
    let z = self.z;
    let c = self.c;
    let t = mul(z, M);
    let u = lo(t);
    let v = hi(t);
    let (w, p) = u.overflowing_add(c);
    self.x = w;
    self.y = x;
    self.z = y;
    self.c = v.wrapping_add(p as u64);
    (y ^ z).wrapping_add(x ^ v)
  }
}

fn warmup() {
  let mut s = 1u64;
  for i in 0 .. 100_000_000 { s = s.wrapping_mul(i); }
  let _: u64 = hint::black_box(s);
}

fn timeit<A, F>(f: F) -> f64 where F: FnOnce() -> A {
  let start = Instant::now();
  let _: A = hint::black_box(f());
  let stop = Instant::now();
  stop.saturating_duration_since(start).as_nanos() as f64
}

fn run_bench<T: BenchRng, F>(name: &str, f: F) where F: Fn(&mut T, usize) -> u64 {
  let mut rng = T::from_seed(*b"abcdefghijklmnop");
  let elapsed = timeit(|| f(&mut rng, COUNT));
  // let rate = ((COUNT as f64) * 1_000.) / elapsed;
  // print!("{:25} {:.3} / μs\n", name, rate);
  print!("{:25} {:.3} ns\n", name, elapsed / (COUNT as f64));
}

fn bench_loop<T: BenchRng>(rng: &mut T, count: usize) -> u64 {
  let mut s = 0u64;
  for _ in 0 .. count {
    s = s.wrapping_add(rng.u64());
  }
  s
}

fn bench_loop_noinline<T: BenchRng>(rng: &mut T, count: usize) -> u64 {
  let mut s = 0u64;
  for _ in 0 .. count {
    s = s.wrapping_add(rng.u64_noinline());
  }
  s
}

#[inline(never)]
fn bench_loop_2x(rng: &mut Firefly, count: usize) -> u64 {
  let mut rng2 = rng.split();
  let mut s = 0u64;
  for _ in 0 .. count / 2 {
    s = s.wrapping_add(rng.u64());
    s = s.wrapping_add(rng2.u64());
  }
  s
}

#[inline(never)]
fn bench_loop_pcg64dxsm(rng: &mut Pcg64dxsm, count: usize) -> u64 {
  bench_loop::<Pcg64dxsm>(rng, count)
}

#[inline(never)]
fn bench_loop_xoroshiro128pp(rng: &mut Xoroshiro128pp, count: usize) -> u64 {
  bench_loop::<Xoroshiro128pp>(rng, count)
}

#[inline(never)]
fn bench_loop_firefly(rng: &mut Firefly, count: usize) -> u64 {
  bench_loop::<Firefly>(rng, count)
}

#[inline(never)]
fn bench_loop_mwc256xxa64(rng: &mut Mwc256xxa64, count: usize) -> u64 {
  bench_loop::<Mwc256xxa64>(rng, count)
}

#[inline(never)]
fn bench_loop_noinline_pcg64dxsm(rng: &mut Pcg64dxsm, count: usize) -> u64 {
  bench_loop_noinline::<Pcg64dxsm>(rng, count)
}

#[inline(never)]
fn bench_loop_noinline_xoroshiro128pp(rng: &mut Xoroshiro128pp, count: usize) -> u64 {
  bench_loop_noinline::<Xoroshiro128pp>(rng, count)
}

#[inline(never)]
fn bench_loop_noinline_firefly(rng: &mut Firefly, count: usize) -> u64 {
  bench_loop_noinline::<Firefly>(rng, count)
}

#[inline(never)]
fn bench_loop_noinline_mwc256xxa64(rng: &mut Mwc256xxa64, count: usize) -> u64 {
  bench_loop_noinline::<Mwc256xxa64>(rng, count)
}

fn main() {
  warmup();
  run_bench("pcg64dxsm", bench_loop_pcg64dxsm);
  run_bench("xoroshiro128++", bench_loop_xoroshiro128pp);
  run_bench("firefly", bench_loop_firefly);
  run_bench("firefly 2x", bench_loop_2x);
  run_bench("mwc256xxa64", bench_loop_mwc256xxa64);
  run_bench("pcg64dxsm (noinline)", bench_loop_noinline_pcg64dxsm);
  run_bench("xoroshiro128++ (noinline)", bench_loop_noinline_xoroshiro128pp);
  run_bench("firefly (noinline)", bench_loop_noinline_firefly);
  run_bench("mwc256xxa64 (noinline)", bench_loop_noinline_mwc256xxa64);
}
