//! # inside-chroot
//!
//! Detect if code is running inside a [chroot environment](https://en.wikipedia.org/wiki/Chroot).
//!
//! ## How does it work
//!
//! Retrieve inode for `/` if it is not `2` assume code runs inside chroot.
//!
//! ## API
//!
//! ```rust
//! use inside_chroot::inside_chroot;
//! let inside = inside_chroot().unwrap();
//! assert!(!inside) // may fail if tests are run inside a chroot env
//! ```
//!
//! ## Credits
//!
//! * https://stackoverflow.com/questions/75182/detecting-a-chroot-jail-from-within
//! * https://unix.stackexchange.com/questions/14345/how-do-i-tell-im-running-in-a-chroot


#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
use std::{fs, io};

#[cfg(target_os = "linux")]
pub fn inside_chroot() -> io::Result<bool> {
    let meta = fs::metadata("/")?;
    let inode = meta.ino();
    Ok(inode != 2)
}
