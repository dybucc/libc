//! Header: `sys/socket.h`
//!
//! * Headers: <https://git.musl-libc.org/cgit/musl/tree/include/sys/socket.h?id=b306b16af15c89a04d8e0c55cac2dadbeb39c083> (official)
//! * Headers: <https://github.com/kraj/musl/blob/a42e9dee266f398026a33d0793c66225c7997755/include/sys/socket.h> (mirror)

use crate::prelude::*;

s! {
    pub struct msghdr {
        pub msg_name: *mut c_void,
        pub msg_namelen: crate::socklen_t,
        pub msg_iov: *mut crate::iovec,
        #[cfg(all(target_pointer_width = "64", target_endian = "big"))]
        __pad1: Padding<c_int>,
        pub msg_iovlen: c_int,
        #[cfg(all(target_pointer_width = "64", target_endian = "little"))]
        __pad1: Padding<c_int>,
        pub msg_control: *mut c_void,
        #[cfg(all(target_pointer_width = "64", target_endian = "big"))]
        __pad2: Padding<c_int>,
        pub msg_controllen: crate::socklen_t,
        #[cfg(all(target_pointer_width = "64", target_endian = "little"))]
        __pad2: Padding<c_int>,
        pub msg_flags: c_int,
    }

    pub struct cmsghdr {
        #[cfg(all(target_pointer_width = "64", target_endian = "big"))]
        pub __pad1: c_int,
        pub cmsg_len: crate::socklen_t,
        #[cfg(all(target_pointer_width = "64", target_endian = "little"))]
        pub __pad1: c_int,
        pub cmsg_level: c_int,
        pub cmsg_type: c_int,
    }
}

extern "C" {
    pub fn sendmmsg(
        sockfd: c_int,
        msgvec: *mut crate::mmsghdr,
        vlen: c_uint,
        flags: c_uint,
    ) -> c_int;
    #[cfg_attr(musl_redir_time64, link_name = "__recvmmsg_time64")]
    pub fn recvmmsg(
        sockfd: c_int,
        msgvec: *mut crate::mmsghdr,
        vlen: c_uint,
        flags: c_uint,
        timeout: *mut crate::timespec,
    ) -> c_int;
}

cfg_if! {
    if #[cfg(any(target_arch = "mips", target_arch = "mips64"))] {
        pub use crate::bits::socket::{
            SOCK_DGRAM,
            SOCK_STREAM,
        };
    } else {
        pub const SOCK_STREAM: c_int = 1;
        pub const SOCK_DGRAM: c_int = 2;
    }
}
