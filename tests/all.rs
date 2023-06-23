use firefly_rng::Rng;
use expect_test::expect;

#[test]
fn test_vectors() {
  let mut a = Rng::from_u64(13);
  let mut b = a.split();
  expect!["0x5abe8cf9689aaeb2"].assert_eq(&format!("{:#018x}", a.next()));
  expect!["0x034f7533bd0a8fdb"].assert_eq(&format!("{:#018x}", a.next()));
  expect!["0x25c5d3aef0f82c3d"].assert_eq(&format!("{:#018x}", a.next()));
  expect!["0x4c0b685ed444086f"].assert_eq(&format!("{:#018x}", a.next()));
  expect!["0x0c6b11c53b2d103b"].assert_eq(&format!("{:#018x}", b.next()));
  expect!["0x5c705dc0aebe4bfe"].assert_eq(&format!("{:#018x}", b.next()));
  expect!["0x8a9d9349f3180539"].assert_eq(&format!("{:#018x}", b.next()));
  expect!["0x9596d7a6f5f69ba1"].assert_eq(&format!("{:#018x}", b.next()));
}
