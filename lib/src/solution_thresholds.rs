use std::env;
use crate::env_names::ENV_SOLUTION_THRESHOLD;

/// Retrieves the solution threshold from the environment variable.
///
/// # Returns
/// The solution threshold as a `usize` parsed from the environment variable `ENV_SOLUTION_THRESHOLD`.
///
/// # Panics
/// This function will panic if:
/// - The environment variable `ENV_SOLUTION_THRESHOLD` is not set.
/// - The value of the environment variable cannot be parsed into a `usize`.
///
/// # Examples
/// ```
/// use std::env;
/// use crate::env_names::ENV_SOLUTION_THRESHOLD;
/// use crate::get_solution_threshold;
///
/// env::set_var(ENV_SOLUTION_THRESHOLD, "42");
/// let threshold = get_solution_threshold();
/// assert_eq!(threshold, 42);
/// ```
pub fn get_solution_threshold() -> usize {
    env::var(ENV_SOLUTION_THRESHOLD).unwrap().trim().parse::<usize>().unwrap()
}
