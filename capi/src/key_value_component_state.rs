//! The state object describes the information to visualize for a key value based component.

use super::{output_str, output_vec, output_has_color, output_color, output_color_or_default};
use livesplit_core::component::key_value::State as KeyValueComponentState;
use livesplit_core::GeneralLayoutSettings;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedKeyValueComponentState = Box<KeyValueComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn KeyValueComponentState_drop(this: OwnedKeyValueComponentState) {
    drop(this);
}

/// The key to visualize.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_key(this: &KeyValueComponentState) -> *const c_char {
    output_str(&this.key)
}

/// The value to visualize.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_value(this: &KeyValueComponentState) -> *const c_char {
    output_str(&this.value)
}

/// The semantic coloring information the value carries.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_semantic_color(
    this: &KeyValueComponentState,
) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", this.semantic_color).unwrap())
}

/// The RGBA color value of the key to visualize, or the layout default if not overridden for this component.
/// This will usually be the preferred function to use.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_key_color_or_default(this: &KeyValueComponentState, settings: &GeneralLayoutSettings) -> u32 {
    output_color_or_default(this.key_color, settings)
}

/// Whether the key has its own color, or uses the default color.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_has_key_color(this: &KeyValueComponentState) -> bool {
    output_has_color(this.key_color)
}

/// The RGBA color value of the key to visualize.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_key_color(this: &KeyValueComponentState) -> u32 {
    output_color(this.key_color.expect("Color is None"))
}

/// The RGBA color value of the value to visualize, or the layout default if not overridden for this component.
/// This will usually be the preferred function to use.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_value_color_or_default(this: &KeyValueComponentState, settings: &GeneralLayoutSettings) -> u32 {
    output_color_or_default(this.value_color, settings)
}

/// Whether the value has its own color, or uses the default color.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_has_value_color(this: &KeyValueComponentState) -> bool {
    output_has_color(this.value_color)
}

/// The RGBA color value of the value to visualize.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_value_color(this: &KeyValueComponentState) -> u32 {
    output_color(this.value_color.expect("Color is None"))
}
