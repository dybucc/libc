#![allow(deprecated)]

#[cfg(freebsd_netlink)]
use libc::netlink::{
    netlink::*,
    netlink_generic::*,
};
#[allow(unused_imports)]
use libc::*;

include!(concat!(env!("OUT_DIR"), "/ctest_output.rs"));
