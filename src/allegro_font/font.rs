use ffi::*;
use allegro::ffi::*;
use allegro::{Color, DrawTarget, Bitmap, BitmapLike};

use std::cast;
use std::libc::*;
use std::mem;

pub enum FontAlignment
{
	AlignLeft,
	AlignCentre,
	AlignRight
}

impl FontAlignment
{
	fn get_allegro_flags(&self) -> c_int
	{
		match *self
		{
			AlignLeft => ALLEGRO_ALIGN_LEFT,
			AlignRight => ALLEGRO_ALIGN_RIGHT,
			AlignCentre => ALLEGRO_ALIGN_CENTRE,
		}
	}
}

pub trait FontDrawing
{
	fn draw_text(&self, font: &Font, color: Color, x: f32, y: f32, align: FontAlignment, text: &str);
	fn draw_justified_text(&self, font: &Font, color: Color, x1: f32, x2: f32, y: f32, diff: f32, align: FontAlignment, text: &str);
}

impl<T: DrawTarget> FontDrawing for T
{
	fn draw_justified_text(&self, font: &Font, color: Color, x1: f32, x2: f32, y: f32, diff: f32, align: FontAlignment, text: &str)
	{
		unsafe
		{
			if al_get_target_bitmap() != self.get_target_bitmap()
			{
				al_set_target_bitmap(self.get_target_bitmap())
			}

			let mut info: ALLEGRO_USTR_INFO = mem::uninit();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *i8, text.len() as c_int);

			al_draw_justified_ustr(cast::transmute(font.get_font()), *color, x1 as c_float,
			                       x2 as c_float, y as c_float, diff as c_float, align.get_allegro_flags(), ustr);
		}
	}

	fn draw_text(&self, font: &Font, color: Color, x: f32, y: f32, align: FontAlignment, text: &str)
	{
		unsafe
		{
			if al_get_target_bitmap() != self.get_target_bitmap()
			{
				al_set_target_bitmap(self.get_target_bitmap())
			}

			let mut info: ALLEGRO_USTR_INFO = mem::uninit();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *i8, text.len() as c_int);

			al_draw_ustr(cast::transmute(font.get_font()), *color, x as c_float, y as c_float, align.get_allegro_flags(), ustr);
		}
	}
}

pub struct Font
{
	priv allegro_font: *mut ALLEGRO_FONT
}

impl Font
{
	fn new(font: *mut ALLEGRO_FONT) -> Option<Font>
	{
		if font.is_null()
		{
			None
		}
		else
		{
			Some(Font{ allegro_font: font })
		}
	}

	pub fn get_font(&self) -> *mut ALLEGRO_FONT
	{
		self.allegro_font
	}

	pub fn get_text_width(&self, text: &str) -> i32
	{
		unsafe
		{
			let mut info: ALLEGRO_USTR_INFO = mem::uninit();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *i8, text.len() as c_int);
			al_get_ustr_width(self.get_font() as *_, ustr) as i32
		}
	}

	pub fn get_line_height(&self) -> i32
	{
		unsafe
		{
			al_get_font_line_height(self.get_font() as *_) as i32
		}
	}

	pub fn get_ascent(&self) -> i32
	{
		unsafe
		{
			al_get_font_ascent(self.get_font() as *_) as i32
		}
	}

	pub fn get_descent(&self) -> i32
	{
		unsafe
		{
			al_get_font_descent(self.get_font() as *_) as i32
		}
	}

	pub fn get_text_dimensions(&self, text: &str) -> (i32, i32, i32, i32)
	{
		unsafe
		{
			let (mut x, mut y, mut w, mut h): (c_int, c_int, c_int, c_int) = mem::uninit();
			let mut info: ALLEGRO_USTR_INFO = mem::uninit();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *i8, text.len() as c_int);
			al_get_ustr_dimensions(self.get_font() as *_, ustr, &mut x, &mut y, &mut w, &mut h);
			(x as i32, y as i32, w as i32, h as i32)
		}
	}
}

impl Drop for Font
{
	fn drop(&mut self)
	{
		unsafe
		{
			al_destroy_font(self.allegro_font);
		}
	}
}

impl ::addon::FontAddon
{
	pub fn create_builtin_font(&self) -> Option<Font>
	{
		Font::new(unsafe
		{
			al_create_builtin_font()
		})
	}

	pub fn load_bitmap_font(&self, filename: &str) -> Option<Font>
	{
		Font::new(unsafe
		{
			filename.with_c_str(|s|
			{
				al_load_bitmap_font(s)
			})
		})
	}

	pub fn load_font(&self, size: i32, filename: &str) -> Option<Font>
	{
		Font::new(unsafe
		{
			filename.with_c_str(|s|
			{
				al_load_font(s, size as c_int, 0)
			})
		})
	}

	pub fn grab_font_from_bitmap(&self, bmp: &Bitmap, ranges: &[(c_int, c_int)]) -> Option<Font>
	{
		Font::new(unsafe
		{
			al_grab_font_from_bitmap(bmp.get_bitmap(), (ranges.len() * 2) as c_int, ranges.as_ptr() as *c_int)
		})
	}
}
