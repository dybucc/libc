//! Source header: `sysdeps/unix/sysv/linux/net/route.h`
//!
//! <https://github.com/sailfishos-mirror/glibc/blob/d6031665c3a59faf75cf6bc55e041611da21d0e6/sysdeps/unix/sysv/linux/net/route.h>

use crate::prelude::*;

s! {
    pub struct rtentry {
        rt_pad1: Padding<c_ulong>,
        pub rt_dst: crate::sockaddr,
        pub rt_gateway: crate::sockaddr,
        pub rt_genmask: crate::sockaddr,
        pub rt_flags: c_ushort,
        rt_pad2: Padding<c_short>,
        rt_pad3: Padding<c_ulong>,
        pub rt_tos: c_uchar,
        pub rt_class: c_uchar,
        #[cfg(target_pointer_width = "64")]
        rt_pad4: Padding<[c_short; 3]>,
        #[cfg(not(target_pointer_width = "64"))]
        rt_pad4: Padding<c_short>,
        pub rt_metric: c_short,
        pub rt_dev: *mut c_char,
        pub rt_mtu: c_ulong,
        pub rt_window: c_ulong,
        pub rt_irtt: c_ushort,
    }
}
