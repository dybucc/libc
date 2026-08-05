//! Header source: `include/unistd.h`
//!
//! * Headers: <https://git.musl-libc.org/cgit/musl/tree/include/unistd.h?id=b306b16af15c89a04d8e0c55cac2dadbeb39c083> (official)
//! * Headers: <https://github.com/kraj/musl/blob/a42e9dee266f398026a33d0793c66225c7997755/include/unistd.h> (mirror)

pub use crate::new::common::posix::unistd::{
    STDERR_FILENO,
    STDIN_FILENO,
    STDOUT_FILENO,
};
