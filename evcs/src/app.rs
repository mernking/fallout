//! EVCS iced Application
//! 
//! The desktop frontend using `iced` 0.14 with a vector canvas widget.
//! 
//! The application supports the minimalist monochrome design palette:
//! - Background: `#F8F9FA` off-white
//! - Primary strokes: `#1A1A1A` charcoal
//! - Accent: `#0066FF` precision blue

use iced::{
    Alignment, Executor, Length, Renderer, Settings,
    widget::{column, text},
    Application, executor,
};

/// The main EVCS iced application window.
/// 
/// Renders the infinite canvas with pan/zoom/rotate controls,
/// tool palettes, and the monochrome design system.
#[derive(Debug, Clone)]
pub struct Ecanvas;

impl Application for Ecanvas {
    type Executor = executor::Default;
    type Message = iced::Message;
    type Flags = ();

    /// Create a new EVCS application instance.
    fn new(_flags: ()) -> (Self, executor::Default) {
        (Ecanvas, executor::Default::default())
    }

    /// Set the window title bar text.
    fn title(&self, _settings: &Settings) -> String {
        "Engineering Vector Canvas System - EVCS".to_string()
    }

    /// Process UI events (currently a no-op; mouse/key events are handled
    /// by the canvas widget in future phases).
    fn update(&mut self, _renderer: &Renderer, _message: Self::Message) {}

    /// Render the EVCS application UI.
    fn view(&self) -> Element<Self::Message> {
        use iced::widget::{column, text};
        use iced::{Alignment, Border, Background};

        // Canvas area placeholder — will be replaced by the
        // `iced::widget::canvas` widget in Phase 1.
        let canvas_placeholder = column![
            text("EVCS — Infinite Vector Canvas").size(28),
            text("— Pan: middle-drag  |  Zoom: scroll  |  Reset: Ctrl+0").size(14),
            text("Color palette: #F8F9FA background, #1A1A1A strokes, #0066FF accent").size(12)
        ]
        .align_x(Alignment::Center);

        canvas_placeholder.into()
    }
}