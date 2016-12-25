#![allow(dead_code)]

extern crate libsoundio_sys as raw;

use super::types::*;
use super::util::*;

use std::os::raw::c_int;
use std::ptr;
use std::cmp::min;

#[derive(Debug)]
pub struct ChannelLayout {
	pub name: String,
	pub channels: Vec<ChannelId>,
}

impl From<raw::SoundIoChannelLayout> for ChannelLayout {
    fn from(layout: raw::SoundIoChannelLayout) -> ChannelLayout {
		ChannelLayout {
			name: latin1_to_string(layout.name),
			channels: layout.channels.iter().take(layout.channel_count as usize).map(|&x| x.into()).collect(),
		}
    }
}

impl From<ChannelLayout> for raw::SoundIoChannelLayout {
    fn from(layout: ChannelLayout) -> raw::SoundIoChannelLayout {
		raw::SoundIoChannelLayout {
			name: ptr::null(), // TODO: Allow the name to be set somehow.
			channel_count: layout.channels.len() as c_int,
			channels: {
				let mut c = [raw::SoundIoChannelId::SoundIoChannelIdInvalid; raw::SOUNDIO_MAX_CHANNELS];
				for i in 0..min(c.len(), layout.channels.len()) {
					c[i] = layout.channels[i].into();
				}
				c
			},
		}
    }
}

// impl ChannelLayout {
// 	// I have this function too because it lets you set the name.
// 	pub fn into_native(&self) -> raw::SoundIoChannelLayout {
// 		raw::SoundIoChannelLayout {
// 			// TODO: I probably need a PhantomData here...
// 			name: self.name.as_ptr() as *const c_char, // TODO: This should probably be Latin1, but I doubt it will cause issues.
// 			channel_count: self.channels.len() as c_int,
// 			channels: {
// 				let mut c = [raw::SoundIoChannelId::SoundIoChannelIdInvalid; raw::SOUNDIO_MAX_CHANNELS];
// 				for i in 0..min(c.len(), self.channels.len()) {
// 					c[i] = self.channels[i].into();
// 				}
// 				c
// 			},
// 		}
// 	}
// }

impl ChannelLayout {
	pub fn get_builtin() -> Vec<ChannelLayout> {
		let count = unsafe { raw::soundio_channel_layout_builtin_count() };
		let mut layouts = Vec::new();
		for i in 0..count {
			layouts.push( unsafe { (*raw::soundio_channel_layout_get_builtin(i)).into() } );
		}
		layouts
	}

	pub fn get_default(channel_count: i32) -> ChannelLayout {
		unsafe {
			(*raw::soundio_channel_layout_get_default(channel_count as c_int)).into()
		}
	}

	pub fn best_matching_channel_layout(_preferred_layouts: &Vec<ChannelLayout>, _available_layouts: &Vec<ChannelLayout>) -> Option<ChannelLayout> {

		unimplemented!();
	}

	// This seems a bit unnecessary.
	pub fn find_channel(&self, channel: ChannelId) -> Option<usize> {

		// There is a C function for this but it seems simpler to do it in Rust.
		self.channels.iter().position(|&c| c == channel)
	}

	// Populate the name field with the built-in name if this layout matches. Returns true if it did.
	pub fn detect_builtin(&mut self) -> bool {

		unimplemented!();
	}

	pub fn sort(_layouts: &mut [ChannelLayout]) {

		unimplemented!();
	}
}