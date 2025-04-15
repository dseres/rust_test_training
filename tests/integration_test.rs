use rtt::*;

#[test]
fn integration_test() {
    let image = crate::parse("0 1\n2 3");
    assert_eq!(
        crate::minmax(&image),
        (0_u8, 3_u8),
        "Min and max should be 0 and 3 on the predefined matrix."
    )
}
