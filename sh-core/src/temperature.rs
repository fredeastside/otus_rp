//! Temperature module with convenient types to measure and convert temperature.
//!
//! This module provides the [`Units`] enum that can represent temperature in either
//! Celsius or Fahrenheit, with easy conversion between units.
//!
//! # Examples
//!
//! ```
//! use sh_core::temperature::Units;
//!
//! // Create a temperature in Celsius
//! let mut temp = Units::Celsius(20.0);
//! println!("{}", temp); // Prints: 20°C
//!
//! // Convert to Fahrenheit
//! temp.fahrenheit();
//! println!("{}", temp); // Prints: 68°F
//!
//! // Chain conversions
//! temp.celsius().fahrenheit();
//! ```
use std::fmt;

/// Temperature unit that can represent either Celsius or Fahrenheit.
///
/// This enum allows storing a temperature value in either unit and provides
/// methods for converting between them.
///
/// # Examples
///
/// ```
/// use sh_core::temperature::Units;
///
/// let temp = Units::Celsius(25.0);
/// assert_eq!(temp.value(), 25.0);
/// ```
#[derive(Debug)]
pub enum Units {
    /// Temperature in Celsius
    Celsius(f64),
    /// Temperature in Fahrenheit
    Fahrenheit(f64),
}

impl Units {
    /// Returns the numeric value of the temperature in its current unit.
    ///
    /// # Examples
    ///
    /// ```
    /// use sh_core::temperature::Units;
    ///
    /// let temp = Units::Celsius(20.0);
    /// assert_eq!(temp.value(), 20.0);
    ///
    /// let temp = Units::Fahrenheit(68.0);
    /// assert_eq!(temp.value(), 68.0);
    /// ```
    #[must_use]
    pub fn value(&self) -> f64 {
        match self {
            Units::Celsius(value) | Units::Fahrenheit(value) => *value,
        }
    }

    /// Converts the temperature to Celsius if it's currently in Fahrenheit.
    ///
    /// If already in Celsius, does nothing. Returns a mutable reference to self
    /// to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use sh_core::temperature::Units;
    ///
    /// let mut temp = Units::Fahrenheit(32.0);
    /// temp.celsius();
    /// assert_eq!(temp.value(), 0.0);
    /// ```
    pub fn celsius(&mut self) -> &mut Self {
        if let Units::Fahrenheit(value) = self {
            *self = Units::Celsius(Self::to_celsius(*value));
        }
        self
    }

    /// Converts the temperature to Fahrenheit if it's currently in Celsius.
    ///
    /// If already in Fahrenheit, does nothing. Returns a mutable reference to self
    /// to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use sh_core::temperature::Units;
    ///
    /// let mut temp = Units::Celsius(0.0);
    /// temp.fahrenheit();
    /// assert_eq!(temp.value(), 32.0);
    /// ```
    pub fn fahrenheit(&mut self) -> &mut Self {
        if let Units::Celsius(value) = self {
            *self = Units::Fahrenheit(Self::to_fahrenheit(*value));
        }
        self
    }

    /// Converts a Celsius value to Fahrenheit.
    fn to_fahrenheit(v: f64) -> f64 {
        (v * 9.0 / 5.0) + 32.0
    }

    /// Converts a Fahrenheit value to Celsius.
    fn to_celsius(v: f64) -> f64 {
        (v - 32.0) * 5.0 / 9.0
    }
}

impl Default for Units {
    fn default() -> Self {
        Self::Celsius(0.0)
    }
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Units::Celsius(value) => write!(f, "{value}°C"),
            Units::Fahrenheit(value) => write!(f, "{value}°F"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units_celsius_value() {
        assert!((Units::Celsius(20.0).value() - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_units_fahrenheit_value() {
        assert!((Units::Fahrenheit(68.0).value() - 68.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!((Units::Celsius(0.0).fahrenheit().value() - 32.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert!((Units::Fahrenheit(32.0).celsius().value() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Units::Celsius(-5.0)), "-5°C");
        assert_eq!(format!("{}", Units::Fahrenheit(65.0)), "65°F");
    }
}
