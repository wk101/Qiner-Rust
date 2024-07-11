use std::mem::size_of;

// Constants

/// Size of the state array in bytes.
pub const STATE_SIZE: usize = 200;

/// Size of the state array in 64-bit words.
pub const STATE_SIZE_64: usize = 200 / size_of::<u64>();

/// Total number of neurons.
pub const NUMBER_OF_NEURONS: usize = 4_194_304;

/// Number of neurons in 64-bit words.
pub const NUMBER_OF_NEURONS_64: usize = NUMBER_OF_NEURONS * size_of::<NeuronLink>() / size_of::<u64>();

/// Bit mask for neuron modulus operations. Used to ensure neuron indices are within valid range.
pub const NEURON_MOD_BITS: u64 = (((NUMBER_OF_NEURONS - 1) << size_of::<NeuronLink>() * 8) | (NUMBER_OF_NEURONS - 1)) as u64;

/// Length of mining data, typically used in mining algorithms.
pub const MINING_DATA_LENGTH: usize = 1024;

/// Number of rounds in the Keccak algorithm for hashing.
pub const KECCAK_ROUND: usize = 12;

/// Number of items in a seed array.
pub const SEED_ITEM_NUM: usize = 32;

/// Character used to split version strings.
pub(crate) const VERSION_SPLIT_CHAR: char = '.';

/// Character used to split random seed strings.
pub(crate) const RANDOM_SEED_SPLIT_CHAR: char = ',';

/// Default port number for network communication.
pub const PORT: u16 = 21841;

/// Default stack size for threads.
pub const STACK_SIZE: usize = 40 * 1024 * 1024;

#[deprecated]
/// Number of neuron values in 64-bit words (deprecated, use `NUMBER_OF_NEURONS_64` instead).
pub const NUMBER_OF_NEURON_VALUES_64: usize = size_of::<NeuronValues>() / size_of::<u64>();

/// Number of items in a nonce array.
pub const NUMBER_OF_NONCE: usize = 32;

/// Number of items in a nonce array in 64-bit words.
pub const NUMBER_OF_NONCE_64: usize = NUMBER_OF_NONCE / size_of::<u64>();

// Types

/// Represents a single item in a seed array.
pub type SeedItem = u8;

/// Represents an array of seed items.
pub type Seed = [SeedItem; SEED_ITEM_NUM];

/// Represents a public key as an array of bytes.
pub type PublicKey = [u8; 32];

/// Represents a nonce as an array of bytes.
pub type Nonce = [u8; NUMBER_OF_NONCE];

/// Represents the state array used in various algorithms.
pub type State = [u8; STATE_SIZE];

/// Represents a single item of mining data.
pub type MiningItemData = u64;

/// Represents the mining data array.
pub type MiningData = [MiningItemData; MINING_DATA_LENGTH];

/// Represents a link between neurons.
pub type NeuronLink = u32;

/// Represents an array of neuron links.
pub type NeuronLinks = [NeuronLink; NUMBER_OF_NEURONS * 2];

/// Represents the value of a single neuron.
pub type NeuronValue = u8;

/// Represents an array of neuron values.
pub type NeuronValues = [NeuronValue; NUMBER_OF_NEURONS];

/// Represents an ID as an array of bytes.
pub type Id = [u8; 60];

/// Represents a signature as an array of 64-bit words.
pub type Signature = [u64; 8];

/// Represents a gamma value as an array of bytes.
pub type Gamma = [u8; 32];

/// Represents a version as an array of bytes.
pub type Version = [u8; 3];

// 64-bit Types

/// Represents an array of seed items in 64-bit words.
pub type Seed64 = [u64; 4];

/// Represents a public key as an array of 64-bit words.
pub type PublicKey64 = [u64; 4];

/// Represents the state array in 64-bit words.
pub type State64 = [u64; STATE_SIZE_64];

/// Represents a nonce as an array of 64-bit words.
pub type Nonce64 = [u64; NUMBER_OF_NONCE_64];

/// Represents a link between neurons in 64-bit words.
pub type NeuronLink64 = u64;

/// Represents an array of neuron links in 64-bit words.
pub type NeuronLinks64 = [NeuronLink64; NUMBER_OF_NEURONS_64 * 2];

/// Represents the value of a single neuron in 64-bit words.
pub type NeuronValue64 = u16;

/// Represents an array of neuron values in 64-bit words.
pub type NeuronValues64 = [NeuronValue64; NUMBER_OF_NEURONS_64];

/// Module for network-related types and constants.
pub mod network {
    use std::mem::size_of;
    use crate::types::NUMBER_OF_NONCE;

    /// Represents a size as an array of bytes.
    pub type Size = [u8; 3];

    /// Represents a protocol identifier.
    pub type Protocol = u8;

    /// Represents a Dejavu identifier as an array of bytes.
    pub type Dejavu = [u8; DEJAVU_ITEM_NUM];

    /// Represents a type identifier.
    pub type Type = u8;

    /// Represents a key as an array of bytes.
    pub type Key = [u8; KEY_ITEM_NUM];

    /// Represents a key as an array of 64-bit words.
    pub type Key64 = [u64; KEY_ITEM_NUM_64];

    /// Represents a combination of key and nonce as an array of bytes.
    pub type KeyAndNonce = [u8; KEY_ITEM_NUM + NUMBER_OF_NONCE];

    // Constants

    /// Number of items in a Dejavu identifier.
    pub const DEJAVU_ITEM_NUM: usize = 3;

    /// Number of items in a key array.
    pub const KEY_ITEM_NUM: usize = 32;

    /// Number of items in a key array in 64-bit words.
    pub const KEY_ITEM_NUM_64: usize = KEY_ITEM_NUM / size_of::<u64>();

    /// Module for protocol-related constants.
    pub mod protocols {
        use crate::types::network::Type;

        /// Identifier for broadcast messages.
        pub const BROADCAST_MESSAGE: Type = 1;
    }
}

