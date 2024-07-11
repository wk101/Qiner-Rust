use std::env;
use crate::env_names::ENV_VERSION;
use crate::types::{VERSION_SPLIT_CHAR, Version};

/// Retrieves the version from the environment variable and parses it into a `Version`.
///
/// # Returns
/// A `Version` parsed from the environment variable `ENV_VERSION`.
///
/// # Panics
/// Panics if the environment variable `ENV_VERSION` is not set or if any of the version components
/// cannot be parsed into a `u8`.
///
/// # Examples
/// ```
/// use std::env;
/// use crate::env_names::ENV_VERSION;
/// use crate::get_version;
///
/// env::set_var(ENV_VERSION, "1.141.0");
/// let version = get_version();
/// assert_eq!(version, [1, 141, 0]);
/// ```
pub fn get_version() -> Version {
    // Retrieve the version string from the environment variable
    let found_version = env::var(ENV_VERSION).unwrap();
    
    // Split the string by the defined split character
    let split = found_version.split(VERSION_SPLIT_CHAR);

    // Initialize a default Version array
    let mut version: Version = Version::default();
    
    // Iterate over the split items and their indices, parsing and assigning each value
    split.into_iter().enumerate().for_each(|(idx, item)| {
        version[idx] = item.trim().parse::<u8>().unwrap();
    });

    version
}

