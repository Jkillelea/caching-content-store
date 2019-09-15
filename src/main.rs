#![allow(warnings)]

extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeStruct};

use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops;
use std::path::PathBuf;
use std::time::{SystemTime,  Instant,  Duration,  UNIX_EPOCH};


// TODO: Look for a better way to hold the update time
// Right now it's just the time since unix epoch
/// Holds some content, an update time, and optionally, a expiry time.
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedContent<T: serde::Serialize> {
    update_time: Duration,
    expires_at: Option<Duration>,
    pub content: T,
}

impl<T: serde::Serialize> CachedContent<T> {
    /// Create a new object from some data. Optionally attach a time the object has to live.
    pub fn from<Dur: Into<Option<Duration>>>(content: T, lifetime: Dur) -> CachedContent<T> {
        let time_now = now();
        let lifetime = lifetime.into();

        let expires_at = if lifetime.is_some() {
            Some(time_now + lifetime.unwrap())
        } else {
            None
        };

        CachedContent {
            update_time: time_now,
            expires_at: expires_at,
            content: content,
        }
    }

    /// Check whether the expiration time has been passed.
    /// If there is no expiration time, `true` is returned.
    pub fn valid(&self) -> bool {
        self.expires_at.map_or(true, |exp| exp <= now())
    }

    pub fn as_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

}

impl<T: serde::Serialize> std::ops::Deref for CachedContent<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.content
    }
}

/// Unix time now
fn now() -> Duration {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
}

fn main() {
    let cached_foo = CachedContent::from("hi there".to_string(), Duration::from_secs(100));
    println!("{:#?}", cached_foo.as_json());

    let cached_foo = CachedContent::from(vec![1, 2, 3, 4, 5], Duration::from_secs(100));
    println!("{:#?}", cached_foo.as_json());
}

// TODO: this is an old implementation of a store for CachedContent. Might want to update
// #[derive(Debug)]
// pub struct CachingContentStore {
//    base_path: PathBuf,
// }
// 
// impl CachingContentStore {
//     pub fn init<P: Into<Option<String>>>(path: P) -> Self {
//         // Default to Home dir
// 
//         let path = path.into(); // Option<String>
// 
//         let mut base_path = if path.is_some() {
//             // user has specified a path to use
//             PathBuf::from(path.unwrap())
//         } else {
//             // default to file in home dir
//             PathBuf::from(env::vars_os()
// 			    .find(|(k, _v)| k == "HOME")
// 			    .unwrap().1)
//         };
// 
//         base_path.push(".rust_caching_content_store");
// 
//         // create if not exsits
//         match fs::create_dir(&base_path) {
//             Err(e) => {
//                 if e.kind() == io::ErrorKind::AlreadyExists {
//                     Ok(())
//                 } else {
//                     Err(e)
//                 }
//             }
//             _ => { Ok(()) }
//         }.unwrap();
// 
//         CachingContentStore {
//             base_path
//         }
//     }
// 
//     // TODO: or maybe this should store types of CachedContent instead of having them
//     // store themselves?
//     pub fn get<T>(&self, tag: String) -> Option<T> {
//         // find by tag
//         // - Nothing -> None
//         // - Something
//         //   - Expired -> Expired
//         //   - Ok -> Valid
// 
//         let read_dir = fs::read_dir(&self.base_path).unwrap();
// 
//         // for f in files {
//         //     match f {
//         //         Ok(file) => {dbg!(file);},
//         //         Err(e)   => {dbg!(e);},
//         //     }
//         // }
// 
//         let files: Vec<fs::DirEntry> = read_dir
//             .filter(|f| f.is_ok())
//             .map(|f| dbg!(f.unwrap()))
//             .collect();
// 
//         None
//     }
// 
//     pub fn set<T>(&mut self, content: CachedContent<T>, tag: &String) -> io::Result<()> {
//         let files: Vec<fs::DirEntry> = fs::read_dir(&self.base_path)?
//             .filter(|f| f.is_ok())
//             .map(|f| f.unwrap())
//             .filter(|f|
//                     &f.file_name()
//                     .into_string()
//                     .unwrap_or("Fake Tag Don't Use".into()) == tag)
//             .collect();
// 
//         dbg!(files);
// 
//         unimplemented!()
//     }
// }

