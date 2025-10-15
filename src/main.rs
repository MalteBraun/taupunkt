use taupunkt::taupunkt;
use std::io;

fn main() {
    let mut input = String::new();

    println!("Bitte Temperatur in °C eingeben:");
    io::stdin().read_line(&mut input).expect("Fehler beim Lesen"); // liest die Eingabe des Benutzers als Text ein
    let temperatur: f64 = input.trim().parse().expect("Ungültige Zahl"); //wandelt die Eingabe in eine Gleitkommazahl (f64)
    input.clear(); //leert den Eingabepuffer

    println!("Bitte relative Luftfeuchtigkeit in % eingeben:");
    io::stdin().read_line(&mut input).expect("Fehler beim Lesen");
    let luftfeuchtigkeit: f64 = input.trim().parse().expect("Ungültige Zahl");

    let taupunkt = taupunkt(temperatur,luftfeuchtigkeit);

    println!("Bei {}°C und einer luftfeuchtigkeit von {}% liegt der Taupunkt bei {:.2}°C", temperatur, luftfeuchtigkeit, taupunkt)
}
