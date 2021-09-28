//! The state object describes the information to visualize for this component.

use super::{output_str, output_vec, output_color};
use livesplit_core::component::timer::State as TimerComponentState;
use livesplit_core::settings::Color;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedTimerComponentState = Box<TimerComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn TimerComponentState_drop(this: OwnedTimerComponentState) {
    drop(this);
}

/// The time shown by the component without the fractional part.
#[no_mangle]
pub extern "C" fn TimerComponentState_time(this: &TimerComponentState) -> *const c_char {
    output_str(&this.time)
}

/// The fractional part of the time shown (including the dot).
#[no_mangle]
pub extern "C" fn TimerComponentState_fraction(this: &TimerComponentState) -> *const c_char {
    output_str(&this.fraction)
}

/// The semantic coloring information the time carries.
#[no_mangle]
pub extern "C" fn TimerComponentState_semantic_color(this: &TimerComponentState) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", this.semantic_color).unwrap())
}

/// The RGBA color value of the timer text.
#[no_mangle]
pub extern "C" fn TimerComponentState_color(this: &TimerComponentState) -> u32 {
    // Top color for now.
    let color = Color {
        red: this.top_color.red,
        green: this.top_color.green,
        blue: this.top_color.blue,
        alpha: this.top_color.alpha,
    };

    output_color(color)
}
