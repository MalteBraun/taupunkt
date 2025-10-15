## Funktion `taupunkt`

Berechnet die Taupunkttemperatur (in °C) basierend auf der aktuellen Lufttemperatur und der relativen Luftfeuchtigkeit (in %). Die Berechnung verwendet die Magnus-Formel mit unterschiedlichen Konstanten für Temperaturen über und unter 0 °C für bessere Genauigkeit. Die Taupunkttemperatur gibt an, bei welcher Temperatur die Luft mit Wasserdampf gesättigt ist und Kondensation beginnt.

**Parameter:**  
- `temperatur` (f64): Lufttemperatur in Grad Celsius  
- `luftfeuchtigkeit` (f64): relative Luftfeuchtigkeit in Prozent  

**Rückgabe:**  
- Taupunkttemperatur in Grad Celsius (f64)
