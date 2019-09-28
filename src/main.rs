#![allow(unused)]
extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};

use std::env;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::{SystemTime, Duration, UNIX_EPOCH};


// TODO: Look for a better way to hold the update time
// Right now it's just the time since unix epoch
/// Holds some content, an update time, and optionally, a expiry time.
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedContent<T: Serialize> {
    update_time: Duration,
    expires_at: Option<Duration>,
    content: T,
}

impl<T: Serialize> CachedContent<T> {
    /// Create a new object from some data. Optionally attach a time the object has to live.
    pub fn from<Dur: Into<Option<Duration>>>(content: T, lifetime: Dur) -> CachedContent<T> {
        let time_now = now();
        let lifetime = lifetime.into();

        CachedContent {
            update_time: time_now,
            expires_at: lifetime.map(|lif| time_now + lif),
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

/// Lets the underlying object be accessed in immutable ways, such as getting an element
/// from an array
impl<T: Serialize> std::ops::Deref for CachedContent<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.content
    }
}

/// Unix time now
fn now() -> Duration {
        SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
}

#[derive(Debug)]
struct CachingContentStore {
   base_path: PathBuf,
}

impl CachingContentStore {
   pub fn init<'a, P>(path: P) -> Self
       where P: Into<Option<&'a str>>
   {
       // Create, if it doesn't alreayd exist, a folder called 
       // `.rust_caching_content_store` under either a specified directory
       // if one is given, or the home directory.
       let mut base_path = path
           .into()
           .map_or_else(
               || { PathBuf::from(env::vars_os().find(|(k, _v)| k == "HOME").unwrap().1) },
               |p| { PathBuf::from(p) });

       base_path.push(".rust_caching_content_store");

       // create if not exsits
       match fs::create_dir(&base_path) {
           Err(e) => {
               if e.kind() == io::ErrorKind::AlreadyExists {
                   Ok(())
               } else {
                   Err(e)
               }
           }
           _ => { Ok(()) }
       }.unwrap();

       CachingContentStore {
           base_path,
       }
   }

   pub fn store<T: Serialize>(&mut self, tag: &str, cached_content: CachedContent<T>) -> io::Result<()> {
       // Serialize and store an object
       let mut path = self.base_path.clone();
       path.push(tag);

       path = dbg!(path);

       let mut f = File::create(path)?;

       f.write_all(cached_content.as_json().unwrap().as_bytes());

       Ok(())
   }

   pub fn get<T: Serialize>(&mut self, tag: &str) -> io::Result<Option<T>> {
       // same deal here. Find a way to find and deserialize objects
       // retrieving an object from JSON is grosser though
       unimplemented!();
   }

}

fn main() {
    let mut store = CachingContentStore::init(".");

    let cached_foo = CachedContent::from("hi there".to_string(), Duration::from_secs(100));
    store.store("foo", cached_foo).unwrap();
}

// impl CachingContentStore {
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

