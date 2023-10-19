use firefly128::Rng;
use expect_test::expect;

#[test]
fn test_vectors() {
  let mut a = Rng::from_u64(13);
  let mut b = a.split();
  expect!["0xdc81271c5ce3195b"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0x2fbbb964d685bc56"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0xce771d2487942820"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0x1c6112ef1f6ffcfc"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0xc4903e56df523b41"].assert_eq(&format!("{:#018x}", b.u64()));
  expect!["0x79632a1755bbe3c1"].assert_eq(&format!("{:#018x}", b.u64()));
  expect!["0x50a516e5e0bac082"].assert_eq(&format!("{:#018x}", b.u64()));
  expect!["0x53d98d6251dda35f"].assert_eq(&format!("{:#018x}", b.u64()));
}
