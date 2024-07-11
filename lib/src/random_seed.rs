use std::env;
use crate::env_names::ENV_RANDOM_SEED;
use crate::types::{RANDOM_SEED_SPLIT_CHAR, Seed, SeedItem};

/// Retrieves the random seed from the environment variable and parses it into a `Seed`.
///
/// # Returns
/// A `Seed` parsed from the environment variable `ENV_RANDOM_SEED`.
///
/// # Panics
/// Panics if the environment variable `ENV_RANDOM_SEED` is not set or if any of the seed items cannot be parsed into a `SeedItem`.
pub fn get_random_seed() -> Seed {
    // Retrieve the random seed string from the environment variable
    let random_seed_string = env::var(ENV_RANDOM_SEED).unwrap();
    
    // Split the string by the defined split character
    let split = random_seed_string.split(RANDOM_SEED_SPLIT_CHAR);
    
    // Initialize a default Seed
    let mut random_seed = Seed::default();
    
    // Iterate over the split items and the seed items, parsing and assigning each value
    for (split_item, seed_item) in split.zip(random_seed.as_mut()) {
        *seed_item = split_item.trim().parse::<SeedItem>().unwrap();
    }
    
    random_seed
}

#[test]
/// Tests the `get_random_seed` function to ensure it correctly parses the environment variable.
fn test_random_seed() {
    // Set the environment variable with a test value
    env::set_var(ENV_RANDOM_SEED, "  126, 27, 26, 27,    26, 27, 26, 27  ");
    
    // Create an expected Seed with the parsed values
    let mut expected_seed: Seed = Seed::default();
    expected_seed[0] = 126;
    expected_seed[1] = 27;
    expected_seed[2] = 26;
    expected_seed[3] = 27;
    expected_seed[4] = 26;
    expected_seed[5] = 27;
    expected_seed[6] = 26;
    expected_seed[7] = 27;

    // Assert that the function output matches the expected Seed
    assert_eq!(expected_seed, get_random_seed());
}
