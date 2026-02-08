use godot::prelude::*;

mod hid;

struct HidLib;

#[gdextension]
unsafe impl ExtensionLibrary for HidLib {}
