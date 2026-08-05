//! Header source: `include/unistd.h`
//!
//! * Headers: <https://gogs.waldemar-brodkorb.de/oss/uclibc-ng/src/60d8e8c0cb9be8a241f6f2645daba260c8aec33c/include/unistd.h> (official)
//! * Headers: <https://github.com/wbx-github/uclibc-ng/blob/60d8e8c0cb9be8a241f6f2645daba260c8aec33c/include/unistd.h> (mirror)

pub use crate::new::common::posix::unistd::{
    STDERR_FILENO,
    STDIN_FILENO,
    STDOUT_FILENO,
};
