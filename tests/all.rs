use firefly_rng::Rng;
use expect_test::expect;

#[test]
fn test_vectors() {
  let mut a = Rng::from_u64(13);
  let mut b = a.split();
  expect!["0x7eb0461209a9194a"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0x0a6fd0da94db9e61"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0xb1eae14208f046e5"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0xdb1446df5e74c2f9"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0xd19c0829420596c4"].assert_eq(&format!("{:#018x}", b.u64()));
  expect!["0x65e7a5c317ca0125"].assert_eq(&format!("{:#018x}", b.u64()));
  expect!["0x1191635c2c36943e"].assert_eq(&format!("{:#018x}", b.u64()));
  expect!["0x3d9651ff7476ee1e"].assert_eq(&format!("{:#018x}", b.u64()));
}
