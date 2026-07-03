//! Defines the core primitives for controlling binary state devices.
//!
//! This module provides a simple abstraction for entities that can be toggled
//! between an "On" and "Off" state, common in home automation or hardware interfaces.

use std::fmt::Display;

/// Represents the operational state of a device.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    /// The device is active or powered on.
    On,
    /// The device is inactive or powered off.
    Off,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::On => write!(f, "On"),
            State::Off => write!(f, "Off"),
        }
    }
}

/// A trait for devices that can be toggled between an [`State::On`] and [`State::Off`] state.
///
/// Implementors must define how the state is mutated via [`on`](Self::on) and
/// [`off`](Self::off), and how to retrieve the current state via [`state`](Self::state).
///
/// # Examples
///
/// Implementing `Switchable` for a simple `LightBulb` struct:
///
/// ```
/// use sh_core::state::{State, Switchable};
///
/// struct LightBulb {
///     current_state: State,
/// }
///
/// impl Switchable for LightBulb {
///     fn on(&mut self) {
///         self.current_state = State::On;
///     }
///
///     fn off(&mut self) {
///         self.current_state = State::Off;
///     }
///
///     fn state(&self) -> State {
///         self.current_state
///     }
/// }
///
/// let mut bulb = LightBulb { current_state: State::Off };
/// bulb.on();
/// assert!(bulb.is_on());
/// ```
pub trait Switchable {
    /// Transitions the device to the [`State::On`] state.
    fn on(&mut self);

    /// Transitions the device to the [`State::Off`] state.
    fn off(&mut self);

    /// Checks if the device is currently in the [`State::On`] state.
    ///
    /// # Returns
    ///
    /// * `true` if the state is `On`.
    /// * `false` if the state is `Off`.
    fn is_on(&self) -> bool {
        matches!(self.state(), State::On)
    }

    /// Retrieves the current [`State`] of the device.
    fn state(&self) -> State;
}
