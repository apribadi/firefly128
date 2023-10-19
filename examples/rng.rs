// Writes a sequence of pseudo-random bytes to stdout.

use std::io::Write;
use firefly128::Rng;

fn main() {
  let mut rng = Rng::from_u64(0);
  let mut out = std::io::stdout().lock();
  let buf = &mut [0u8; 4096];

  loop {
    rng.fill_u8(buf);
    if let Err(_) = out.write_all(buf) { break; }
  }
}
