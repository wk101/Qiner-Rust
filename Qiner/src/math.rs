use lib::types::{KECCAK_ROUND, Nonce64, PublicKey64, State64, STATE_SIZE_64};

/// Generates a random sequence of 64-bit unsigned integers based on the given public key and nonce.
///
/// # Arguments
/// * `public_key` - A reference to the public key used for generating the random sequence.
/// * `nonce` - A reference to the nonce used for generating the random sequence.
/// * `output` - A mutable reference to an array where the generated random sequence will be stored.
///
/// # Type Parameters
/// * `S` - The size of the output array.
///
/// # Example
/// ```
/// use lib::types::{PublicKey64, Nonce64};
/// let public_key: PublicKey64 = [0; 64];
/// let nonce: Nonce64 = [0; 32];
/// let mut output: [u64; 4] = [0; 4];
/// random_64(&public_key, &nonce, &mut output);
/// ```
pub(crate) fn random_64<const S: usize>(public_key: &PublicKey64, nonce: &Nonce64, output: &mut [u64; S]) {
    // Initialize the state array with default values
    let mut state: State64 = State64::default();

    // Copy the public key into the beginning of the state array
    state[..public_key.len()].copy_from_slice(public_key);

    // Copy the nonce into the state array immediately following the public key
    state[public_key.len()..public_key.len() + nonce.len()].copy_from_slice(nonce);

    // Split the output array into chunks of the size of the state array
    let mut chunks_mut = output.chunks_mut(STATE_SIZE_64);

    // Process each chunk by applying the keccak-p1600 permutation
    while let Some(chunk) = chunks_mut.next() {
        // Apply the keccak-p1600 permutation to the state array
        keccak::p1600(&mut state, KECCAK_ROUND);

        // Copy the resulting state array into the current chunk of the output array
        chunk.clone_from_slice(&state[..chunk.len()]);
    }
}
