use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::LazyLock;

// These are included by default in `bracket-terminal`.
const TERMINAL_8_8_BYTES: &[u8] = include_bytes!("../resources/terminal8x8.png");
const TERMINAL_8_16_BYTES: &[u8] = include_bytes!("../resources/vga8x16.png");

pub static EMBED: LazyLock<Mutex<Dictionary>> = LazyLock::new(|| Mutex::new(Dictionary::new()));

/// Stores a dictionary of resources, generally added via `embedded_resource!` and `link_resource!` macros.
#[derive(Default)]
pub struct Dictionary {
    entries: HashMap<String, &'static [u8]>,
}

impl Dictionary {
    /// Create a new, empty dictionary.
    #[must_use]
    pub fn new() -> Dictionary {
        let mut dict = Dictionary {
            entries: HashMap::new(),
        };
        dict.add_resource("resources/terminal8x8.png".to_string(), TERMINAL_8_8_BYTES);
        dict.add_resource("resources/vga8x16.png".to_string(), TERMINAL_8_16_BYTES);
        dict
    }

    /// Request a resource, returning either a byte array or `None`.
    #[must_use]
    pub fn get_resource(&self, path: String) -> Option<&'static [u8]> {
        let fixed_path = if std::path::MAIN_SEPARATOR == '/' {
            path
        } else {
            path.replace(std::path::MAIN_SEPARATOR, "/")
        };

        if self.entries.contains_key(&fixed_path) {
            return Some(self.entries[&fixed_path]);
        }
        None
    }

    /// Insert a resource into the dictionary.
    pub fn add_resource(&mut self, path: String, bytes: &'static [u8]) {
        self.entries.insert(path, bytes);
    }
}
