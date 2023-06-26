use firefly_rng::Rng;
use expect_test::expect;

#[test]
fn test_vectors() {
  let mut a = Rng::from_u64(13);
  let mut b = a.split();
  expect!["0xcb9f84b320d21708"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0xe1f8ac48ae06f0eb"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0x159c1a0c3798e21a"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0x4bcc060cf405a7fd"].assert_eq(&format!("{:#018x}", a.u64()));
  expect!["0x02d5f796e9035187"].assert_eq(&format!("{:#018x}", b.u64()));
  expect!["0x68eac07086daf10e"].assert_eq(&format!("{:#018x}", b.u64()));
  expect!["0xb1edb62eced9a218"].assert_eq(&format!("{:#018x}", b.u64()));
  expect!["0x2462cf7076796f25"].assert_eq(&format!("{:#018x}", b.u64()));
}
