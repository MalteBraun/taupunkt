pub fn taupunkt(temperatur: f64, luftfeuchtigkeit: f64) -> f64 {
    let (a, b) = if temperatur >= 0.0 {
        (17.62, 243.12) // wenn die Temperatur >= 0°C ist
    } else {
        (22.46, 272.62) // wenn die Temperatur <= 0°C ist
    };

    let rh = luftfeuchtigkeit / 100.0;
    let alpha = rh.ln() + (a * temperatur) / (b + temperatur);
    b * alpha / (a - alpha)
}