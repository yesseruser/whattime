use crate::time::Time;

#[test]
fn to_string() {
    let time1 = Time::new(0, 0, 0);
    let time2 = Time::new(1, 1, 1);
    let time3 = Time::new(12, 56, 4);
    assert_eq!(time1.to_string(), "00:00:00");
    assert_eq!(time2.to_string(), "01:01:01");
    assert_eq!(time3.to_string(), "12:56:04");
}
