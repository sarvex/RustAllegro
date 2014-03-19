#[crate_id="allegro5#5.0.10.1"];

#[comment = "Allegro 5 core library Rust bindings"];
#[license = "zlib"];
#[crate_type = "lib"];
#[feature(globs)];
#[feature(macro_rules)];
#[feature(struct_variant)];

pub use internal::bitmap::external::*;
pub use internal::bitmap_like::*;
pub use internal::color::*;
pub use internal::core::*;
pub use internal::core_drawing::*;
pub use internal::display::*;
pub use internal::events::external::*;
pub use internal::joystick::*;
pub use internal::keycodes::*;
pub use internal::run::*;
pub use internal::timer::*;
pub use rust_util::*;

#[macro_escape]
#[path = "../macros.rs"]
pub mod macros;

pub mod rust_util;
pub mod ffi;

mod internal
{
	pub mod bitmap;
	pub mod bitmap_like;
	pub mod color;
	pub mod core;
	pub mod core_drawing;
	pub mod display;
	pub mod events;
	pub mod joystick;
	pub mod keycodes;
	pub mod run;
	pub mod timer;
}