#![cfg(windows)]

#[test]
fn test() {
    #[cfg(all(target_arch = "x86", target_env = "gnu"))]
    assert_eq!(size_of::<libc::time_t>(), 4);
    #[cfg(not(all(target_arch = "x86", target_env = "gnu")))]
    assert_eq!(size_of::<libc::time_t>(), 8);
}
