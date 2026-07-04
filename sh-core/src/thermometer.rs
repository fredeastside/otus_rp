use crate::state::{State, Switchable};
use crate::temperature::Units;
use std::fmt;

/// A smart thermometer that measures temperature.
#[derive(Debug)]
pub struct Thermometer {
    name: String,
    value: Units,
    state: State,
}

impl Thermometer {
    /// Creates a new `Thermometer` with a given initial temperature. It is off by default.
    #[must_use]
    pub fn new(name: &str, value: Units) -> Self {
        Self {
            name: name.to_string(),
            value,
            state: State::Off,
        }
    }

    /// Returns the kind of the device, always `"Thermometer"`.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        "Thermometer"
    }

    /// Returns the thermometer's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Converts the thermometer's reading to Celsius.
    pub fn celsius(&mut self) {
        self.value.celsius();
    }

    /// Converts the thermometer's reading to Fahrenheit.
    pub fn fahrenheit(&mut self) {
        self.value.fahrenheit();
    }

    /// Returns a reference to the current temperature reading.
    #[must_use]
    pub fn value(&self) -> &Units {
        &self.value
    }
}

impl Switchable for Thermometer {
    fn on(&mut self) {
        self.state = State::On;
    }

    fn off(&mut self) {
        self.state = State::Off;
    }

    fn state(&self) -> State {
        self.state
    }
}

impl fmt::Display for Thermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state {
            State::On => write!(
                f,
                "{} ({}): {}, Temp: {}",
                self.kind(),
                self.name(),
                self.state,
                self.value
            ),
            State::Off => write!(f, "{} ({}): {}", self.kind(), self.name(), self.state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_thermometer_is_off() {
        let thermo = Thermometer::new("thermometer1", Units::Celsius(20.0));
        assert!(!thermo.is_on());
    }

    #[test]
    fn test_thermometer_can_be_turned_on() {
        let mut thermo = Thermometer::new("thermometer1", Units::Celsius(20.0));
        thermo.on();
        assert!(thermo.is_on());
    }

    #[test]
    fn test_thermometer_can_be_turned_off() {
        let mut thermo = Thermometer::new("thermometer1", Units::Celsius(20.0));
        thermo.on();
        thermo.off();
        assert!(!thermo.is_on());
    }

    #[test]
    fn test_celsius_conversion() {
        let mut thermo = Thermometer::new("thermometer1", Units::Fahrenheit(32.0));
        thermo.celsius();
        assert!((thermo.value().value() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_fahrenheit_conversion() {
        let mut thermo = Thermometer::new("thermometer1", Units::Celsius(0.0));
        thermo.fahrenheit();
        assert!((thermo.value().value() - 32.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_display_when_off() {
        let thermo = Thermometer::new("thermometer1", Units::Celsius(25.0));
        assert_eq!(format!("{thermo}"), "Thermometer (thermometer1): Off");
    }

    #[test]
    fn test_display_when_on() {
        let mut thermo = Thermometer::new("thermometer1", Units::Celsius(25.0));
        thermo.on();
        assert_eq!(
            format!("{thermo}"),
            "Thermometer (thermometer1): On, Temp: 25°C"
        );
    }
}
