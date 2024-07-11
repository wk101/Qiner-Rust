use std::arch::x86_64::{_rdrand32_step, _rdrand64_step};
use std::mem::{size_of, transmute, transmute_copy, zeroed};
use std::ptr;
use k12::digest::{ExtendableOutputReset, Update};
use k12::KangarooTwelve;
use lib::types::network::{Dejavu, Key, KeyAndNonce, Protocol, Size, Type};
use lib::types::{Gamma, Nonce, Nonce64, NUMBER_OF_NONCE, NUMBER_OF_NONCE_64, PublicKey64, Signature};
use lib::version::get_version;

/// Struct representing the header of a request/response.
#[derive(Default, Debug, Clone, Copy)]
pub struct RequestResponseHeader {
    size: Size,
    protocol: Protocol,
    dejavu: Dejavu,
    r#type: Type,
}

impl RequestResponseHeader {
    /// Creates a new `RequestResponseHeader`.
    ///
    /// # Arguments
    /// * `in_type` - The type of the request/response.
    /// * `in_size` - The size of the request/response.
    ///
    /// # Returns
    /// A new `RequestResponseHeader`.
    pub fn new(in_type: &Type, in_size: &usize) -> Self {
        let mut header: RequestResponseHeader = Default::default();
        header.set_size(in_size);
        header.set_protocol();
        header.zeroed_dejavu();
        header.set_type(in_type);

        header
    }

    /// Gets the size of the request/response.
    ///
    /// # Returns
    /// The size of the request/response.
    pub fn get_size(&self) -> usize {
        unsafe {
            ptr::read_unaligned(&self.size as *const Size as *const usize)
        }
    }

    /// Sets the size of the request/response.
    ///
    /// # Arguments
    /// * `new_size` - The new size of the request/response.
    pub fn set_size(&mut self, new_size: &usize) {
        unsafe {
            self.size = transmute_copy::<usize, Size>(new_size);
        }
    }

    /// Gets the protocol version.
    ///
    /// # Returns
    /// The protocol version.
    pub fn get_protocol(&self) -> Protocol {
        self.protocol
    }

    /// Sets the protocol version to the current version.
    pub fn set_protocol(&mut self) {
        let version = get_version();
        self.protocol = version[1];
    }

    /// Checks if the dejavu field is zeroed.
    ///
    /// # Returns
    /// `true` if the dejavu field is zeroed, `false` otherwise.
    pub fn is_dejavu_zero(&self) -> bool {
        self.dejavu.iter().all(|item| *item == 0u8)
    }

    /// Zeroes the dejavu field.
    pub fn zeroed_dejavu(&mut self) {
        unsafe {
            self.dejavu = zeroed::<Dejavu>();
        }
    }

    /// Randomizes the dejavu field using a random 32-bit integer.
    pub fn randomize_dejavu(&mut self) {
        assert!(size_of::<Dejavu>() <= size_of::<u32>());

        let mut random: u32 = 0;
        unsafe { _rdrand32_step(&mut random) };

        unsafe {
            self.dejavu = transmute_copy::<u32, Dejavu>(&random);
        }
    }

    /// Gets the type of the request/response.
    ///
    /// # Returns
    /// The type of the request/response.
    pub fn get_type(&self) -> Type {
        self.r#type
    }

    /// Sets the type of the request/response.
    ///
    /// # Arguments
    /// * `new_type` - The new type of the request/response.
    pub fn set_type(&mut self, new_type: &Type) {
        self.r#type = *new_type;
    }
}

/// Struct representing a message.
#[derive(Default, Debug, Copy, Clone)]
pub struct Message {
    source_public_key: PublicKey64,
    destination_public_key: PublicKey64,
    gamming_nonce: Nonce64,
}

impl Message {
    /// Gets the gamming nonce of the message.
    ///
    /// # Returns
    /// The gamming nonce.
    pub fn get_gamming_nonce(&self) -> Nonce64 {
        self.gamming_nonce
    }
}

/// Struct representing a packet.
#[derive(Debug, Clone, Copy)]
pub struct Packet {
    header: RequestResponseHeader,
    message: Message,
    solution_nonce: Nonce64,
    signature: Signature,
}

impl Packet {
    /// Creates a new `Packet`.
    ///
    /// # Arguments
    /// * `r#type` - The type of the packet.
    /// * `computor_public_key` - The public key of the computor.
    /// * `in_nonce` - The nonce to be used in the packet.
    ///
    /// # Returns
    /// A new `Packet`.
    pub fn new(r#type: &Type, computor_public_key: &PublicKey64, in_nonce: &Nonce64) -> Self {
        //*****************************
        // Header
        //*****************************

        let header: RequestResponseHeader = RequestResponseHeader::new(r#type, &size_of::<Packet>());

        //*****************************
        // Message
        //*****************************

        let mut message = Message::default();
        message.source_public_key = PublicKey64::default();
        message.destination_public_key = *computor_public_key;

        let mut kangaroo_twelve = KangarooTwelve::default();

        let mut shared_key_and_gamming_nonce: KeyAndNonce = unsafe { zeroed::<KeyAndNonce>() };
        let mut gamming_key: Key = Key::default();
        let mut nonce_buffer: Nonce = Nonce::default();

        let nonce_chunk_size = NUMBER_OF_NONCE / NUMBER_OF_NONCE_64;
        loop {
            nonce_buffer.chunks_mut(nonce_chunk_size).for_each(|items| {
                let item_64 = items.as_mut_ptr() as *mut u64;
                unsafe {
                    _rdrand64_step(item_64.as_mut().unwrap());
                }
            });

            shared_key_and_gamming_nonce[(gamming_key.len())..].copy_from_slice(nonce_buffer.as_slice());

            kangaroo_twelve.update(shared_key_and_gamming_nonce.as_slice());
            kangaroo_twelve.finalize_xof_reset_into(gamming_key.as_mut());

            if (gamming_key[0]) == 0 {
                break;
            }
        }
        message.gamming_nonce = unsafe { transmute::<Nonce, Nonce64>(nonce_buffer) };

        //*****************************
        // Solution nonce
        //*****************************

        // Get Gamma
        let mut gamma: Gamma = Gamma::default();
        kangaroo_twelve.update(gamming_key.as_slice());
        kangaroo_twelve.finalize_xof_reset_into(gamma.as_mut_slice());

        // Make solution nonce 
        let nonce_u8_ptr = in_nonce.as_ptr() as *const Nonce;
        unsafe {
            nonce_buffer.iter_mut().zip(nonce_u8_ptr.read().iter()).zip(gamma.as_slice()).for_each(|((nonce_buffer_value, in_nonce_value), gamma_value)| {
                *nonce_buffer_value = *in_nonce_value ^ *gamma_value;
            });
        }
        let solution_nonce = unsafe { transmute::<Nonce, Nonce64>(nonce_buffer) };

        //*****************************
        // Signature
        //*****************************
        let signature = Packet::get_random_signature();

        //*****************************
        // Packet
        //*****************************

        Packet {
            header,
            message,
            solution_nonce,
            signature,
        }
    }

    /// Generates a random signature.
    ///
    /// # Returns
    /// A random `Signature`.
    pub fn get_random_signature() -> Signature {
        let mut signature = Signature::default();
        signature.iter_mut().for_each(|item: &mut u64| {
            unsafe {
                _rdrand64_step(item);
            }
        });

        signature
    }
}
