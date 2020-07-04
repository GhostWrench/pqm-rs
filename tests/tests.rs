use pqm_rs::Quantity;

#[test]
fn add_quantities() {
    let q1 = Quantity::new(1.0, [1,0,0,0,0,0,0]);
    let q2 = Quantity::new(1.0, [1,0,0,0,0,0,0]);
    let expected = Quantity::new(2.0, [1,0,0,0,0,0,0]);
    let result = q1.add(&q2);
    assert!(result.eq(&expected));
}

#[test]
#[should_panic]
fn add_unlike() {
    let q1 = Quantity::new(1.0, [1,0,0,0,0,0,0]);
    let q2 = Quantity::new(1.0, [0,1,0,0,0,0,0]);
    let _result = q1.add(&q2);
}
