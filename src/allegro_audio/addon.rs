// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::kinds::marker::NoSend;

use sync::{Arc, Mutex};

use allegro::Core;
use ffi::allegro_audio::*;

static mut initialized: bool = false;
//#[thread_local]
static mut spawned_on_this_thread: bool = false;

pub struct AudioAddon
{
	no_send_marker: NoSend,
	core_mutex: Arc<Mutex<()>>,
}

impl AudioAddon
{
	pub fn init(core: &Core) -> Option<AudioAddon>
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
					Some(AudioAddon{ no_send_marker: NoSend, core_mutex: core.get_core_mutex() })
				}
			}
			else
			{
				if al_install_audio() != 0
				{
					initialized = true;
					spawned_on_this_thread = true;
					Some(AudioAddon{ no_send_marker: NoSend, core_mutex: core.get_core_mutex() })
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
			al_get_allegro_audio_version() as i32
		}
	}

	pub fn get_core_mutex(&self) -> Arc<Mutex<()>>
	{
		self.core_mutex.clone()
	}
}
