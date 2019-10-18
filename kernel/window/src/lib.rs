//! This crate defines a `Window` trait.
//! A window manager maintains a list of `Window` objects.

#![no_std]

extern crate event_types;
extern crate dfqueue;
extern crate frame_buffer;

use event_types::Event;
use dfqueue::{DFQueueProducer};
use frame_buffer::Coord;

/// Trait for windows. A window manager holds a list of objects who implement the `Window` trait.
/// A `Window` provides states required by the window manager such as the size, the loaction and the active state of a window.
pub trait Window {
    /// Clears the window on the screen including the border and padding.
    fn clear(&self) -> Result<(), &'static str>;

    /// Checks if the coordinate relative to the top-left corner of the window is within it exluding the border and padding.
    fn contains(&self, coordinate: Coord) -> bool;

    /// Sets the window as active or not.
    fn set_active(&mut self, active: bool) -> Result<(), &'static str>;

    /// Draws the border of the window.
    fn draw_border(&self, color: u32) -> Result<(), &'static str>;

    /// Adjusts the size (width, height) and coordinate of the window relative to the top-left corner of the screen.
    fn resize(
        &mut self,
        coordinate: Coord,
        width: usize,
        height: usize,
    ) -> Result<(usize, usize), &'static str>;

    /// Gets the size of content without padding.
    fn get_content_size(&self) -> (usize, usize);

    /// Gets the coordinate of content relative to top-left corner of the window without padding.
    fn get_content_position(&self) -> Coord;

    /// Gets the producer of events.
    fn events_producer(&mut self) -> &mut DFQueueProducer<Event>;
}