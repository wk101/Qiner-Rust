use k12::digest::{ExtendableOutput, Update};
use k12::KangarooTwelve;
use lib::types::{Id, PublicKey, PublicKey64};

const A: u8 = 'A' as u8;

/// Converts an `Id` to a `PublicKey64`.
///
/// # Arguments
/// * `id` - The `Id` to be converted.
/// * `public_key` - A mutable reference to a `PublicKey64` where the result will be stored.
///
/// # Returns
/// `true` if the conversion is successful, `false` otherwise.
pub fn get_public_key_64_from_id(id: &Id, public_key: &mut PublicKey64) -> bool {
    *public_key = Default::default();

    for i in 0..4 {
        for j in (0..14).rev() {
            let id_value = id[i * 14 + j];
            // Check if the ID value is within the range 'A' to 'Z'
            if id_value < 'A' as u8 || id_value > 'Z' as u8 {
                *public_key = Default::default();
                return false;
            }

            let delta_id_value = (id_value - A) as u64;
            public_key[i] = public_key[i] * 26u64 + delta_id_value;
        }
    }

    true
}

/// Converts a `PublicKey64` to an `Id`.
///
/// # Arguments
/// * `public_key` - The `PublicKey64` to be converted.
/// * `id` - A mutable reference to an `Id` where the result will be stored.
pub fn get_id_from_public_key_64(public_key: &PublicKey64, id: &mut Id) {
    for i in 0..4 {
        let mut public_key_fragment = public_key[i];
        for j in 0..14 {
            let id_idx = i * 14usize + j;
            id[id_idx] = (public_key_fragment % 26u64 + ('A' as u64)) as u8;
            public_key_fragment /= 26;
        }
    }

    // Calculate the Identity Bytes Checksum
    let mut identity_bytes_checksum: u32;
    {
        let mut kangaroo_twelve = KangarooTwelve::default();
        let ptr_public_key_8 = public_key.as_ptr() as *const PublicKey;
        unsafe {
            // Update the hash with the public key
            kangaroo_twelve.update(&ptr_public_key_8.read());

            // Finalize the hash and obtain the first 3 bytes of the output
            let mut result: [u8; 3] = Default::default();
            kangaroo_twelve.finalize_xof_into(&mut result);
            // Combine the 3 bytes into a single 24-bit integer
            identity_bytes_checksum = result[0] as u32 | (result[1] as u32) << 8 | (result[2] as u32) << 16;
        }
    }

    // Mask to fit within 18 bits
    identity_bytes_checksum &= 0x3FFFF;
    for i in 0..4 {
        // Convert the checksum to characters 'A' to 'Z' and store in the ID
        id[56 + i] = (identity_bytes_checksum % 26 + 'A' as u32) as u8;
        identity_bytes_checksum /= 26;
    }
}
