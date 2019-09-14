#![allow(warnings)]

extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeStruct};

use std::time::{Instant, Duration};

// Holds some content, an update time,
// and optionally, a expiry time.
// TODO: find a good time unit that can be serialized into JSON
// right now it's just unixtime
#[derive(Debug, Serialize)]
struct CachedContent<T> {
    update_time: usize,
    expires_at: Option<usize>,
    content: T,
}

impl<T> CachedContent<T> {
    pub fn from<OptUnix: Into<Option<usize>>>(content: T, expiry: OptUnix) -> CachedContent<T> {
        CachedContent {
            update_time: 0,
            expires_at: expiry.into(),
            content: content,
        }
    }
}

fn main() {
    let cached_foo = CachedContent::from(0i32, None);
    println!("{:?}", cached_foo);
}
