use rustgames::graphics::effects::animation::easing::Easing;

#[test]
fn linear_identity() {
    assert_eq!(Easing::Linear.apply(0.0), 0.0);
    assert_eq!(Easing::Linear.apply(0.5), 0.5);
    assert_eq!(Easing::Linear.apply(1.0), 1.0);
}

#[test]
fn ease_in_below_linear() {
    assert!(Easing::EaseIn.apply(0.5) < 0.5);
}

#[test]
fn ease_out_above_linear() {
    assert!(Easing::EaseOut.apply(0.5) > 0.5);
}

#[test]
fn clamped_outside_range() {
    assert_eq!(Easing::Linear.apply(-1.0), 0.0);
    assert_eq!(Easing::Linear.apply(2.0), 1.0);
}

#[test]
fn bounce_reaches_one() {
    assert!((Easing::Bounce.apply(1.0) - 1.0).abs() < 0.01);
}

#[test]
fn elastic_at_boundaries() {
    assert_eq!(Easing::Elastic.apply(0.0), 0.0);
    assert_eq!(Easing::Elastic.apply(1.0), 1.0);
}
