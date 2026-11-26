use ordering_system::Money;

#[test]
fn create_money() {
    let money = Money::new(100);

    assert_eq!(100, money.as_minor_units());
}

#[test]
fn create_zero_money() {
    let money = Money::new(0);

    assert_eq!(0, money.as_minor_units());
}

#[test]
fn checked_add_success() {
    let first = Money::new(100);
    let second = Money::new(200);

    let result = first.checked_add(second);

    assert_eq!(result, Some(Money::new(300)));
}

#[test]
fn checked_add_overflow() {
    let money = Money::new(u64::MAX);

    assert_eq!(money.checked_add(Money::new(1)), None);
}

#[test]
fn checked_mul_success() {
    let money = Money::new(100);
    let quantity = 3;

    assert_eq!(Some(Money::new(300)), money.checked_mul(quantity));
}

#[test]
fn checked_mul_overflow() {
    let money = Money::new(u64::MAX);
    let quantity = 2;

    assert_eq!(None, money.checked_mul(quantity));
}

#[test]
fn checked_mul_zero() {
    let money = Money::new(100);
    let quantity = 0;

    assert_eq!(Some(Money::new(0)), money.checked_mul(quantity));
}

#[test]
fn maximum_valid_value() {
    let money = Money::new(u64::MAX);

    assert_eq!(money.checked_add(Money::new(0)), Some(Money::new(u64::MAX)));
}
