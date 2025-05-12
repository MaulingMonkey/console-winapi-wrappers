//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/console/console-structures)\]

mod ascii_or_unicode_char;              pub use ascii_or_unicode_char::*;
mod char_info;                          pub use char_info::*;
mod color_ref;                          pub use color_ref::*;
mod console_cursor_info;                pub use console_cursor_info::*;
mod console_screen_buffer_info;         pub use console_screen_buffer_info::*;
mod console_screen_buffer_info_ex;      pub use console_screen_buffer_info_ex::*;
mod coord;                              pub use coord::*;
mod input_record;                       pub use input_record::*;
mod input_record_event_type;            pub use input_record_event_type::*;
mod small_rect;                         pub use small_rect::*;
