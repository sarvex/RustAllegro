// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_id="allegro_dialog#5.0.10.1"]

#![comment = "Allegro 5 native dialog addon Rust bindings"]
#![license = "zlib"]
#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(thread_local)]

extern crate allegro = "allegro5#5.0.10.1";
extern crate libc;

use allegro::{Core, Flag, Display};
use ffi::allegro_dialog::*;

use std::option::Some;
use std::kinds::marker::NoSend;

#[macro_escape]
#[path = "../macros.rs"]
pub mod macros;

pub mod ffi
{
	pub use self::allegro_dialog::*;
	pub mod allegro_dialog
	{
		#![allow(non_camel_case_types)]

		use libc::*;
		use allegro::c_bool;
		use allegro::ffi::{ALLEGRO_DISPLAY, ALLEGRO_EVENT_SOURCE};

		opaque!(ALLEGRO_FILECHOOSER)
		opaque!(ALLEGRO_TEXTLOG)

		pub static ALLEGRO_FILECHOOSER_FILE_MUST_EXIST: u32 = 1;
		pub static ALLEGRO_FILECHOOSER_SAVE: u32 = 2;
		pub static ALLEGRO_FILECHOOSER_FOLDER: u32 = 4;
		pub static ALLEGRO_FILECHOOSER_PICTURES: u32 = 8;
		pub static ALLEGRO_FILECHOOSER_SHOW_HIDDEN: u32 = 16;
		pub static ALLEGRO_FILECHOOSER_MULTIPLE: u32 = 32;

		pub static ALLEGRO_MESSAGEBOX_WARN: u32 = 1;
		pub static ALLEGRO_MESSAGEBOX_ERROR: u32 = 2;
		pub static ALLEGRO_MESSAGEBOX_OK_CANCEL: u32 = 4;
		pub static ALLEGRO_MESSAGEBOX_YES_NO: u32 = 8;
		pub static ALLEGRO_MESSAGEBOX_QUESTION: u32 = 16;

		pub static ALLEGRO_TEXTLOG_NO_CLOSE: u32 = 1;
		pub static ALLEGRO_TEXTLOG_MONOSPACE: u32 = 2;

		pub static ALLEGRO_EVENT_NATIVE_DIALOG_CLOSE: c_uint = 600;

		#[link(name = "allegro_dialog")]
		extern "C"
		{
			pub fn al_init_native_dialog_addon() -> c_bool;
			pub fn al_shutdown_native_dialog_addon();
			pub fn al_create_native_file_dialog(initial_path: *c_char, title: *c_char, patterns: *c_char, mode: c_int) -> *mut ALLEGRO_FILECHOOSER;
			pub fn al_show_native_file_dialog(display: *mut ALLEGRO_DISPLAY, dialog: *mut ALLEGRO_FILECHOOSER) -> c_bool;
			pub fn al_get_native_file_dialog_count(dialog: *ALLEGRO_FILECHOOSER) -> c_int;
			pub fn al_get_native_file_dialog_path(dialog: *ALLEGRO_FILECHOOSER, index: size_t) -> *c_char;
			pub fn al_destroy_native_file_dialog(dialog: *mut ALLEGRO_FILECHOOSER);
			pub fn al_show_native_message_box(display: *mut ALLEGRO_DISPLAY, title: *c_char, heading: *c_char, text: *c_char, buttons: *c_char, flags: c_int) -> c_int;
			pub fn al_open_native_text_log(title: *c_char, flags: c_int) -> *mut ALLEGRO_TEXTLOG;
			pub fn al_close_native_text_log(textlog: *mut ALLEGRO_TEXTLOG);
			pub fn al_append_native_text_log(textlog: *mut ALLEGRO_TEXTLOG, format: *c_char, ...);
			pub fn al_get_native_text_log_event_source(textlog: *mut ALLEGRO_TEXTLOG) -> *mut ALLEGRO_EVENT_SOURCE;
			pub fn al_get_allegro_native_dialog_version() -> uint32_t;
		}
	}
}

flag_type!(
	MessageBoxFlags
	{
		MESSAGEBOX_WARN =      ALLEGRO_MESSAGEBOX_WARN,
		MESSAGEBOX_ERROR =     ALLEGRO_MESSAGEBOX_ERROR,
		MESSAGEBOX_OK_CANCEL = ALLEGRO_MESSAGEBOX_OK_CANCEL,
		MESSAGEBOX_YES_NO =    ALLEGRO_MESSAGEBOX_YES_NO,
		MESSAGEBOX_QUESTION =  ALLEGRO_MESSAGEBOX_QUESTION
	}
)

pub enum MessageBoxResult
{
	NoButton,
	Affirmative,
	Negatory
}

static mut initialized: bool = false;
#[thread_local]
static mut spawned_on_this_thread: bool = false;

pub struct DialogAddon
{
	no_send_marker: NoSend
}

impl DialogAddon
{
	pub fn init(core: &Core) -> Option<DialogAddon>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread
				{
					None
				}
				else
				{
					spawned_on_this_thread = true;
					Some(DialogAddon{ no_send_marker: NoSend })
				}
			}
			else
			{
				if al_init_native_dialog_addon() != 0
				{
					initialized = true;
					spawned_on_this_thread = true;
					Some(DialogAddon{ no_send_marker: NoSend })
				}
				else
				{
					None
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_native_dialog_version() as i32
		}
	}
}

pub fn show_native_message_box(display: Option<&Display>, title: &str, heading: &str, text: &str, buttons: Option<&str>, flags: MessageBoxFlags) -> MessageBoxResult
{
	use std::ptr;
	use libc::c_int;

	let d = display.map_or(ptr::mut_null(), |d| d.get_allegro_display());
	let ret = title.with_c_str(|title|
	{
		heading.with_c_str(|heading|
		{
			text.with_c_str(|text|
			{
				unsafe
				{
					match buttons
					{
						Some(buttons) => buttons.with_c_str(|buttons|
						{
							al_show_native_message_box(d, title, heading, text, buttons, flags.get() as c_int)
						}),
						None =>
						{
							al_show_native_message_box(d, title, heading, text, ptr::null(), flags.get() as c_int)
						}
					}
				}
			})
		})
	});
	match ret
	{
		1 => Affirmative,
		2 => Negatory,
		_ => NoButton,
	}
}