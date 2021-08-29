//! The state object describes the information to visualize for this component.

use super::{output_str, output_has_color, output_color, output_color_or_default};
use livesplit_core::component::text::{State as TextComponentState, TextState};
use livesplit_core::GeneralLayoutSettings;
use std::os::raw::c_char;

/// type
pub type OwnedTextComponentState = Box<TextComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn TextComponentState_drop(this: OwnedTextComponentState) {
    drop(this);
}

/// Accesses the left part of the text. If the text isn't split up, an empty
/// string is returned instead.
#[no_mangle]
pub extern "C" fn TextComponentState_left(this: &TextComponentState) -> *const c_char {
    if let TextState::Split(left, _) = &this.text {
        output_str(left)
    } else {
        output_str("")
    }
}

/// Accesses the right part of the text. If the text isn't split up, an empty
/// string is returned instead.
#[no_mangle]
pub extern "C" fn TextComponentState_right(this: &TextComponentState) -> *const c_char {
    if let TextState::Split(_, right) = &this.text {
        output_str(right)
    } else {
        output_str("")
    }
}

/// Accesses the centered text. If the text isn't centered, an empty string is
/// returned instead.
#[no_mangle]
pub extern "C" fn TextComponentState_center(this: &TextComponentState) -> *const c_char {
    if let TextState::Center(center) = &this.text {
        output_str(center)
    } else {
        output_str("")
    }
}

/// Returns whether the text is split up into a left and right part.
#[no_mangle]
pub extern "C" fn TextComponentState_is_split(this: &TextComponentState) -> bool {
    matches!(this.text, TextState::Split(_, _))
}

/// The RGBA color value of the first part of the text, as in the left or center parts. 
/// Returns the layout default if the color hasn't been overridden for this component.
/// This will usually be the preferred function to use.
#[no_mangle]
pub extern "C" fn TextComponentState_left_center_color_or_default(this: &TextComponentState, settings: &GeneralLayoutSettings) -> u32 {
    output_color_or_default(this.left_center_color, settings)
}

/// Whether the first part of the text has its own, overriden color, or uses the default color from the layout.
#[no_mangle]
pub extern "C" fn TextComponentState_has_left_center_color(this: &TextComponentState) -> bool {
    output_has_color(this.left_center_color)
}

/// The RGBA color value of the first part of the text.
/// Do not call this unless you know that this component overrides the layout default.
#[no_mangle]
pub extern "C" fn TextComponentState_left_center_color(this: &TextComponentState) -> u32 {
    output_color(this.left_center_color.expect("Color is None"))
}

/// The RGBA color value of the second part of the text, as in the right part, when split. 
/// Returns the layout default if the color hasn't been overridden for this component.
/// This will usually be the preferred function to use.
#[no_mangle]
pub extern "C" fn TextComponentState_right_color_or_default(this: &TextComponentState, settings: &GeneralLayoutSettings) -> u32 {
    output_color_or_default(this.right_color, settings)
}

/// Whether the second part of the text has its own, overriden color, or uses the default color from the layout.
#[no_mangle]
pub extern "C" fn TextComponentState_has_right_color(this: &TextComponentState) -> bool {
    output_has_color(this.right_color)
}

/// The RGBA color value of the second part of the text.
/// Do not call this unless you know that this component overrides the layout default.
#[no_mangle]
pub extern "C" fn TextComponentState_right_color(this: &TextComponentState) -> u32 {
    output_color(this.right_color.expect("Color is None"))
}
