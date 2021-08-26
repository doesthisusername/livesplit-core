//! The general settings of the layout that apply to all components.

use super::output_color;
use livesplit_core::GeneralLayoutSettings;

/// type
pub type OwnedGeneralLayoutSettings = Box<GeneralLayoutSettings>;

/// Creates a default general layout settings configuration.
#[no_mangle]
pub extern "C" fn GeneralLayoutSettings_default() -> OwnedGeneralLayoutSettings {
    Box::new(GeneralLayoutSettings::default())
}

/// drop
#[no_mangle]
pub extern "C" fn GeneralLayoutSettings_drop(this: OwnedGeneralLayoutSettings) {
    drop(this);
}

/// The RGBA color value of the regular layout text.
#[no_mangle]
pub extern "C" fn GeneralLayoutSettings_text_color(this: &GeneralLayoutSettings) -> u32 {
    output_color(this.text_color)
}
