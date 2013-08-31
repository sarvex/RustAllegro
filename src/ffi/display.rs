use std::libc::*;

use ffi::events::ALLEGRO_EVENT_SOURCE;
use ffi::bitmap::ALLEGRO_BITMAP;
use rust_util::c_bool;

pub static ALLEGRO_WINDOWED: u32                  = 1 << 0;
pub static ALLEGRO_FULLSCREEN: u32                = 1 << 1;
pub static ALLEGRO_OPENGL: u32                    = 1 << 2;
pub static ALLEGRO_DIRECT3D_INTERNAL: u32         = 1 << 3;
pub static ALLEGRO_RESIZABLE: u32                 = 1 << 4;
pub static ALLEGRO_FRAMELESS: u32                 = 1 << 5;
pub static ALLEGRO_NOFRAME: u32                   = ALLEGRO_FRAMELESS;
pub static ALLEGRO_GENERATE_EXPOSE_EVENTS: u32    = 1 << 6;
pub static ALLEGRO_OPENGL_3_0: u32                = 1 << 7;
pub static ALLEGRO_OPENGL_FORWARD_COMPATIBLE: u32 = 1 << 8;
pub static ALLEGRO_FULLSCREEN_WINDOW: u32         = 1 << 9;
pub static ALLEGRO_MINIMIZED: u32                 = 1 << 10;

pub static ALLEGRO_RED_SIZE: u32 = 0;
pub static ALLEGRO_GREEN_SIZE: u32 = 1;
pub static ALLEGRO_BLUE_SIZE: u32 = 2;
pub static ALLEGRO_ALPHA_SIZE: u32 = 3;
pub static ALLEGRO_RED_SHIFT: u32 = 4;
pub static ALLEGRO_GREEN_SHIFT: u32 = 5;
pub static ALLEGRO_BLUE_SHIFT: u32 = 6;
pub static ALLEGRO_ALPHA_SHIFT: u32 = 7;
pub static ALLEGRO_ACC_RED_SIZE: u32 = 8;
pub static ALLEGRO_ACC_GREEN_SIZE: u32 = 9;
pub static ALLEGRO_ACC_BLUE_SIZE: u32 = 10;
pub static ALLEGRO_ACC_ALPHA_SIZE: u32 = 11;
pub static ALLEGRO_STEREO: u32 = 12;
pub static ALLEGRO_AUX_BUFFERS: u32 = 13;
pub static ALLEGRO_COLOR_SIZE: u32 = 14;
pub static ALLEGRO_DEPTH_SIZE: u32 = 15;
pub static ALLEGRO_STENCIL_SIZE: u32 = 16;
pub static ALLEGRO_SAMPLE_BUFFERS: u32 = 17;
pub static ALLEGRO_SAMPLES: u32 = 18;
pub static ALLEGRO_RENDER_METHOD: u32 = 19;
pub static ALLEGRO_FLOAT_COLOR: u32 = 20;
pub static ALLEGRO_FLOAT_DEPTH: u32 = 21;
pub static ALLEGRO_SINGLE_BUFFER: u32 = 22;
pub static ALLEGRO_SWAP_METHOD: u32 = 23;
pub static ALLEGRO_COMPATIBLE_DISPLAY: u32 = 24;
pub static ALLEGRO_UPDATE_DISPLAY_REGION: u32 = 25;
pub static ALLEGRO_VSYNC: u32 = 26;
pub static ALLEGRO_MAX_BITMAP_SIZE: u32 = 27;
pub static ALLEGRO_SUPPORT_NPOT_BITMAP: u32 = 28;
pub static ALLEGRO_CAN_DRAW_INTO_BITMAP: u32 = 29;
pub static ALLEGRO_SUPPORT_SEPARATE_ALPHA: u32 = 30;
pub static ALLEGRO_DISPLAY_OPTIONS_COUNT: u32 = 31;

pub static ALLEGRO_DONTCARE: u32 = 0;
pub static ALLEGRO_REQUIRE: u32 = 1;
pub static ALLEGRO_SUGGEST: u32 = 2;

pub static ALLEGRO_DISPLAY_ORIENTATION_0_DEGREES: u32 = 0;
pub static ALLEGRO_DISPLAY_ORIENTATION_90_DEGREES: u32 = 1;
pub static ALLEGRO_DISPLAY_ORIENTATION_180_DEGREES: u32 = 2;
pub static ALLEGRO_DISPLAY_ORIENTATION_270_DEGREES: u32 = 3;
pub static ALLEGRO_DISPLAY_ORIENTATION_FACE_UP: u32 = 4;
pub static ALLEGRO_DISPLAY_ORIENTATION_FACE_DOWN: u32 = 5;

pub struct ALLEGRO_DISPLAY;

externfn!(fn al_set_new_display_refresh_rate(refresh_rate: c_int))
externfn!(fn al_set_new_display_flags(flags: c_int))
externfn!(fn al_get_new_display_refresh_rate() -> c_int)
externfn!(fn al_get_new_display_flags() -> c_int)

externfn!(fn al_get_display_width(display: *mut ALLEGRO_DISPLAY) -> c_int)
externfn!(fn al_get_display_height(display: *mut ALLEGRO_DISPLAY) -> c_int)
externfn!(fn al_get_display_format(display: *mut ALLEGRO_DISPLAY) -> c_int)
externfn!(fn al_get_display_refresh_rate(display: *mut ALLEGRO_DISPLAY) -> c_int)
externfn!(fn al_get_display_flags(display: *mut ALLEGRO_DISPLAY) -> c_int)
externfn!(fn al_set_display_flag(display: *mut ALLEGRO_DISPLAY, flag: c_int, onoff: c_bool) -> c_bool)

externfn!(fn al_create_display(w: c_int, h: c_int) -> *mut ALLEGRO_DISPLAY)
externfn!(fn al_destroy_display(display: *mut ALLEGRO_DISPLAY))
externfn!(fn al_get_current_display() -> *mut ALLEGRO_DISPLAY)
externfn!(fn al_set_target_bitmap(bitmap: *mut ALLEGRO_BITMAP))
externfn!(fn al_set_target_backbuffer(display: *mut ALLEGRO_DISPLAY))
externfn!(fn al_get_backbuffer(display: *mut ALLEGRO_DISPLAY) -> *mut ALLEGRO_BITMAP)
externfn!(fn al_get_target_bitmap() -> *mut ALLEGRO_BITMAP)

externfn!(fn al_acknowledge_resize(display: *mut ALLEGRO_DISPLAY) -> c_bool)
externfn!(fn al_resize_display(display: *mut ALLEGRO_DISPLAY, width: c_int, height: c_int) -> c_bool)
externfn!(fn al_flip_display())
externfn!(fn al_update_display_region(x: c_int, y: c_int, width: c_int, height: c_int))
externfn!(fn al_is_compatible_bitmap(bitmap: *mut ALLEGRO_BITMAP) -> c_bool)

externfn!(fn al_wait_for_vsync() -> c_bool)

externfn!(fn al_get_display_event_source(display: *mut ALLEGRO_DISPLAY) -> *mut ALLEGRO_EVENT_SOURCE)

externfn!(fn al_set_display_icon(display: *mut ALLEGRO_DISPLAY, icon: *mut ALLEGRO_BITMAP))
externfn!(fn al_set_display_icons(display: *mut ALLEGRO_DISPLAY, num_icons: c_int, icons: *mut *mut ALLEGRO_BITMAP))

externfn!(fn al_get_new_display_adapter() -> c_int)
externfn!(fn al_set_new_display_adapter(adapter: c_int))
externfn!(fn al_set_new_window_position(x: c_int, y: c_int))
externfn!(fn al_get_new_window_position(x: *mut c_int, y: *mut c_int))
externfn!(fn al_set_window_position(display: *mut ALLEGRO_DISPLAY, x: c_int, y: c_int))
externfn!(fn al_get_window_position(display: *mut ALLEGRO_DISPLAY, x: *mut c_int, y: *mut c_int))

externfn!(fn al_set_window_title(display: *mut ALLEGRO_DISPLAY, title: *c_char))

externfn!(fn al_set_new_display_option(option: c_int, value: c_int, importance: c_int))
externfn!(fn al_get_new_display_option(option: c_int, importance: *mut c_int) -> c_int)
externfn!(fn al_reset_new_display_options())
externfn!(fn al_get_display_option(display: *mut ALLEGRO_DISPLAY, option: c_int) -> c_int)

externfn!(fn al_hold_bitmap_drawing(hold: c_bool))
externfn!(fn al_is_bitmap_drawing_held() -> c_bool)
