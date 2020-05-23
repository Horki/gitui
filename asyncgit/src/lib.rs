//! asyncgit

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::result_unwrap_used)]
#![deny(clippy::panic)]

mod diff;
mod error;
mod revlog;
mod status;
pub mod sync;

pub use crate::{
    diff::{AsyncDiff, DiffParams},
    revlog::{AsyncLog, FetchStatus},
    status::{AsyncStatus, StatusParams},
    sync::{
        diff::{DiffLine, DiffLineType, FileDiff},
        status::{StatusItem, StatusItemType},
        utils::is_repo,
    },
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// this type is used to communicate events back through the channel
#[derive(Copy, Clone, Debug)]
pub enum AsyncNotification {
    ///
    Status,
    ///
    Diff,
    ///
    Log,
}

/// current working director `./`
pub static CWD: &str = "./";

/// helper function to calculate the hash of an arbitrary type that implements the `Hash` trait
pub fn hash<T: Hash + ?Sized>(v: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}
