//! Header: `sysdeps/x86/nptl/bits/struct_mutex.h`
//!
//! <https://github.com/sailfishos-mirror/glibc/blob/4a07bb292f921c10e71fbf48c4a7f44391feb06c/sysdeps/x86/nptl/bits/struct_mutex.h>

use crate::prelude::*;

pub const fn __PTHREAD_MUTEX_INITIALIZER(__kind: c_int) -> crate::pthread_mutex_t {
    // We don't need the whole complicated `__pthread_mutex_s` definition, just use the
    // offset of the `__kind` field.
    let kind_offset = if cfg!(target_pointer_width = "64") {
        4 * size_of::<c_int>()
    } else {
        3 * size_of::<c_int>()
    };

    let size = [0; crate::__SIZEOF_PTHREAD_MUTEX_T];
    let kind_bytes = __kind.to_ne_bytes();
    let repl = u8_slice_cast_char_slice(&kind_bytes);
    let size = replace_array_items(size, repl, kind_offset);
    crate::pthread_mutex_t { size }
}
