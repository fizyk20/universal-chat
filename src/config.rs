use crate::core::EventType;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use toml::{self, Value};

/// Structure representing the configuration along with
/// the path to the file where it is saved
#[derive(Clone)]
pub struct Config<T> {
    path: PathBuf,
    inner: ConfigInner<T>,
}

/// A definition of an event source
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceDef {
    pub source_type: String,
    pub config: Option<Value>,
}

/// A definition of a module
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleDef {
    pub module_type: String,
    pub config: Option<Value>,
    pub priority: u8,
    pub subscriptions: HashMap<String, Vec<EventType>>,
}

/// Inner structure with configuration data, read by Serde from a file
/// in JSON format
#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigInner<T> {
    pub log_folder: String,
    pub sources: HashMap<String, SourceDef>,
    pub modules: HashMap<String, ModuleDef>,
    pub custom: T,
}

impl<T> Config<T> {
    /// Loads configuration from a file and returns the resulting Config object
    pub fn new<P: AsRef<Path>>(path: P) -> Config<T>
    where
        T: DeserializeOwned,
    {
        let path_buf = path.as_ref().to_path_buf();
        let mut file = fs::File::open(path)
            .ok()
            .expect(&format!("Couldn't open file {:?}", path_buf));
        let mut config = String::new();
        file.read_to_string(&mut config)
            .expect("Couldn't read from file");
        Config {
            path: path_buf,
            inner: toml::from_str(&config).expect("Config is an invalid TOML file"),
        }
    }
}

impl<T> Deref for Config<T> {
    type Target = ConfigInner<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Config<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.inner
    }
}

/// A global object to access the configuration
#[macro_export]
macro_rules! config {
    ($T:ty, $fname:expr) => {
        lazy_static! {
            pub static ref CONFIG: ::std::sync::Mutex<$crate::Config<$T>> =
                ::std::sync::Mutex::new($crate::Config::new($fname));
        }
    };
}
