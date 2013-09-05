use std::libc::*;

use ffi::color::ALLEGRO_COLOR;
use rust_util::c_bool;

pub struct ALLEGRO_BITMAP;

pub static ALLEGRO_MEMORY_BITMAP: u32 = 1;
pub static ALLEGRO_KEEP_BITMAP_FORMAT: u32 = 2;
pub static ALLEGRO_FORCE_LOCKING: u32 = 4;
pub static ALLEGRO_NO_PRESERVE_TEXTURE: u32 = 8;
pub static ALLEGRO_ALPHA_TEST: u32 = 16;
pub static _ALLEGRO_INTERNAL_OPENGL: u32 = 32;
pub static ALLEGRO_MIN_LINEAR: u32 = 64;
pub static ALLEGRO_MAG_LINEAR: u32 = 128;
pub static ALLEGRO_MIPMAP: u32 = 256;
pub static ALLEGRO_NO_PREMULTIPLIED_ALPHA: u32 = 512;
pub static ALLEGRO_VIDEO_BITMAP: u32 = 1024;

externfn!(fn al_set_new_bitmap_format(format: c_int))
externfn!(fn al_set_new_bitmap_flags(flags: c_int))
externfn!(fn al_get_new_bitmap_format() -> c_int)
externfn!(fn al_get_new_bitmap_flags() -> c_int)
externfn!(fn al_add_new_bitmap_flag(flag: c_int))

externfn!(fn al_get_bitmap_width(bitmap: *mut ALLEGRO_BITMAP) -> c_int)
externfn!(fn al_get_bitmap_height(bitmap: *mut ALLEGRO_BITMAP) -> c_int)
externfn!(fn al_get_bitmap_format(bitmap: *mut ALLEGRO_BITMAP) -> c_int)
externfn!(fn al_get_bitmap_flags(bitmap: *mut ALLEGRO_BITMAP) -> c_int)

externfn!(fn al_create_bitmap(w: c_int, h: c_int) -> *mut ALLEGRO_BITMAP)
externfn!(fn al_destroy_bitmap(bitmap: *mut ALLEGRO_BITMAP))

externfn!(fn al_put_pixel(x: c_int, y: c_int, color: ALLEGRO_COLOR))
externfn!(fn al_put_blended_pixel(x: c_int, y: c_int, color: ALLEGRO_COLOR))
externfn!(fn al_get_pixel(bitmap: *mut ALLEGRO_BITMAP, x: c_int, y: c_int) -> ALLEGRO_COLOR)

externfn!(fn al_convert_mask_to_alpha(bitmap: *mut ALLEGRO_BITMAP, mask_color: ALLEGRO_COLOR))

externfn!(fn al_set_clipping_rectangle(x: c_int, y: c_int, width: c_int, height: c_int))
externfn!(fn al_reset_clipping_rectangle())
externfn!(fn al_get_clipping_rectangle(x: *mut c_int, y: *mut c_int, w: *mut c_int, h: *mut c_int))

externfn!(fn al_create_sub_bitmap(parent: *mut ALLEGRO_BITMAP, x: c_int, y: c_int, w: c_int, h: c_int) -> *mut ALLEGRO_BITMAP)
externfn!(fn al_is_sub_bitmap(bitmap: *mut ALLEGRO_BITMAP) -> c_bool)
externfn!(fn al_get_parent_bitmap(bitmap: *mut ALLEGRO_BITMAP) -> *mut ALLEGRO_BITMAP)

externfn!(fn al_clone_bitmap(bitmap: *mut ALLEGRO_BITMAP) -> *mut ALLEGRO_BITMAP)