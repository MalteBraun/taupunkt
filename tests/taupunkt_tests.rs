use taupunkt::taupunkt;

#[test]
fn test_taupunkt_above_zero() {
    let temperatur = 22.0;
    let luftfeuchtigkeit = 60.0;
    let tp = taupunkt(temperatur, luftfeuchtigkeit);
    let expected = 13.8;
    let epsilon = 0.1;
    assert!((tp - expected).abs() < epsilon, "Taupunkt sollte ca. 13.8 sein, war {}", tp);
}

#[test]
fn test_taupunkt_below_zero() {
    let temperatur = -5.0;
    let luftfeuchtigkeit = 80.0;
    let tp = taupunkt(temperatur, luftfeuchtigkeit);
    let expected = -7.58; // Näherungswert
    let epsilon = 0.2;
    assert!((tp - expected).abs() < epsilon, "Taupunkt sollte ca. -7.58 sein, war {}", tp);
}

#[test]
fn test_taupunkt_zero_humidity() {
    let temperatur = 25.0;
    let luftfeuchtigkeit = 0.1;  // Nahe 0 %, praktisch trockene Luft
    let tp = taupunkt(temperatur, luftfeuchtigkeit);
    assert!(tp < -50.0, "Taupunkt soll sehr niedrig sein bei fast 0 % Luftfeuchtigkeit");
}

#[test]
fn test_taupunkt_1() {
    let temperatur = 60.0;
    let luftfeuchtigkeit = 60.0;
    let tp = taupunkt(temperatur, luftfeuchtigkeit);
    let expected = 49.43;
    let epsilon = 0.2;
    assert!((tp - expected).abs() < epsilon, "Taupunkt sollte ca. 49.43 sein, war {}", tp);
}

#[test]
fn test_taupunkt_2() {
    let temperatur = -60.0;
    let luftfeuchtigkeit = 60.0;
    let tp = taupunkt(temperatur, luftfeuchtigkeit);
    let expected = -63.71;
    let epsilon = 0.2;
    assert!((tp - expected).abs() < epsilon, "Taupunkt sollte ca. -63.71 sein, war {}", tp);
}

#[test]
fn test_taupunkt_3() {
    let temperatur = 12.21;
    let luftfeuchtigkeit = 32.71;
    let tp = taupunkt(temperatur, luftfeuchtigkeit);
    let expected = -3.73;
    let epsilon = 0.2;
    assert!((tp - expected).abs() < epsilon, "Taupunkt sollte ca. -3.73 sein, war {}", tp);
}