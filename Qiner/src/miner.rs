use std::arch::x86_64::_rdrand64_step;
use std::collections::HashMap;
use std::mem::{size_of, zeroed};
use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::thread::ThreadId;
use lib::solution_threshold::get_solution_threshold;
use lib::types::{
    MiningItemData,
    MiningData,
    NeuronLink,
    NeuronLinks64,
    NeuronValue,
    NeuronValues,
    Nonce64,
    PublicKey64,
    Seed,
    Seed64,
    MINING_DATA_LENGTH,
    NEURON_MOD_BITS,
    NUMBER_OF_NEURONS,
    NUMBER_OF_NEURONS_64,
};

/// Container for neuron data specific to each thread
#[derive(Debug, Clone, Default)]
pub struct NeuronContainer {
    neuron_data: HashMap<ThreadId, NeuronData>,
}

impl NeuronContainer {
    /// Retrieve or initialize neuron data for the given thread
    ///
    /// # Arguments
    /// * `thread_id` - A reference to the ThreadId for which to retrieve the neuron data
    ///
    /// # Returns
    /// A mutable reference to the NeuronData associated with the provided thread ID
    pub fn get_mut_data(&mut self, thread_id: &ThreadId) -> &mut NeuronData {
        self.neuron_data.entry(thread_id.clone()).or_default()
    }
}

/// Structure holding neuron links and values
#[derive(Debug, Clone, Default)]
pub struct NeuronData {
    neuron_links: NeuronLinks64,
    neuron_values: NeuronValues,
}

impl NeuronData {
    pub fn new() -> Self {
        NeuronData {
            neuron_links: [0; NUMBER_OF_NEURONS_64 * 2],
            neuron_values: [NeuronValue::MAX; NUMBER_OF_NEURONS],
        }
    }
}

/// Main mining structure
#[derive(Debug, Clone)]
pub struct Miner {
    solution_threshold: usize,
    num_threads: usize,
    mining_data: MiningData,
    public_key: PublicKey64,
    score_counter: Arc<AtomicUsize>,
    iteration_counter: Arc<AtomicUsize>,
    pub found_nonce: Arc<tokio::sync::Mutex<Vec<Nonce64>>>,
}

impl Miner {
    /// Constructor to create a new Miner instance
    ///
    /// # Arguments
    /// * `public_key` - A PublicKey64 used for generating neuron links
    /// * `num_threads` - The number of threads to be used in the mining process
    ///
    /// # Returns
    /// A new instance of the Miner struct
    pub fn new(public_key: PublicKey64, num_threads: usize) -> Self {
        // Generate a random seed for mining data initialization
        let random_seed = Miner::generate_random_seed();

        // Initialize mining data with zeroes
        let mut mining_data: MiningData;
        unsafe {
            mining_data = zeroed::<MiningData>();
        }

        // Generate mining data based on the random seed
        crate::math::random_64(&random_seed, &random_seed, &mining_data);

        Miner {
            solution_threshold: get_solution_threshold(),
            num_threads,
            mining_data,
            public_key,
            score_counter: Arc::new(AtomicUsize::new(0)),
            iteration_counter: Arc::new(AtomicUsize::new(0)),
            found_nonce: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }

    /// Get the current score
    ///
    /// # Returns
    /// The current score as a usize
    pub fn get_score(&self) -> usize {
        self.score_counter.load(Ordering::SeqCst)
    }

    /// Get the current iteration count
    ///
    /// # Returns
    /// The current iteration count as a usize
    pub fn get_iteration_count(&self) -> usize {
        self.iteration_counter.load(Ordering::SeqCst)
    }

    /// Generate a random 64-bit seed using the RDRAND instruction
    ///
    /// # Returns
    /// A 64-bit seed of type Seed64
    fn generate_random_seed() -> Seed64 {
        let seed = lib::random_seed::get_random_seed();
        unsafe { std::mem::transmute(seed) }
    }

    /// Find a solution using the provided nonce and neuron data
    ///
    /// # Arguments
    /// * `nonce` - A mutable reference to a Nonce64 for storing the generated nonce
    /// * `neuron_data` - A mutable reference to NeuronData for storing neuron links and values
    ///
    /// # Returns
    /// A boolean indicating whether a solution was found
    pub fn find_solution(&self, nonce: &mut Nonce64, neuron_data: &mut NeuronData) -> bool {
        // Generate a random nonce
        nonce.iter_mut().for_each(|item| { *item = generate_random_u64(); });

        // Generate neuron links based on public key and nonce
        crate::math::random_64(&self.public_key, nonce, &mut neuron_data.neuron_links);

        // Mask neuron links to fit neuron mod bits
        for idx in 0..NUMBER_OF_NEURONS_64 {
            neuron_data.neuron_links[idx] &= NEURON_MOD_BITS;
            neuron_data.neuron_links[NUMBER_OF_NEURONS_64 + idx] &= NEURON_MOD_BITS;
        }

        // Mining logic with neuron values and mining data
        let mut remaining_iterations = MINING_DATA_LENGTH;
        let mut score: usize = 0;

        loop {
            let prev_value0 = neuron_data.neuron_values[NUMBER_OF_NEURONS - 1];
            let prev_value1 = neuron_data.neuron_values[NUMBER_OF_NEURONS - 2];

            for idx in 0..NUMBER_OF_NEURONS_64 {
                let left_idx = idx * 2;
                let right_idx = idx * 2 + 1;

                let left_neuron0 = (neuron_data.neuron_links[left_idx] as NeuronLink) as usize;
                let right_neuron0 = ((neuron_data.neuron_links[left_idx] >> size_of::<NeuronLink>() * 8) as NeuronLink) as usize;

                let left_neuron1 = (neuron_data.neuron_links[right_idx] as NeuronLink) as usize;
                let right_neuron1 = ((neuron_data.neuron_links[right_idx] >> size_of::<NeuronLink>() * 8) as NeuronLink) as usize;

                let and_result0 = neuron_data.neuron_values[left_neuron0] & neuron_data.neuron_values[right_neuron0];
                let and_result1 = neuron_data.neuron_values[left_neuron1] & neuron_data.neuron_values[right_neuron1];
                neuron_data.neuron_values[left_idx] = !(and_result0);
                neuron_data.neuron_values[right_idx] = !(and_result1);
            }

            let current_value0 = neuron_data.neuron_values[NUMBER_OF_NEURONS - 1];
            let current_value1 = neuron_data.neuron_values[NUMBER_OF_NEURONS - 2];

            let mining_data_chunk = self.mining_data[score >> 6];
            let bit_is_set = ((mining_data_chunk >> (score & 63) as MiningItemData) & 1) as u8;
            if current_value0 != prev_value0 && current_value1 == prev_value1 {
                if bit_is_set == 0 {
                    break;
                }
                score += 1;
            } else if current_value1 != prev_value1 && current_value0 == prev_value0 {
                if bit_is_set == 1 {
                    break;
                }
                score += 1;
            } else {
                remaining_iterations -= 1;
                if remaining_iterations == 0 {
                    break;
                }
            }
        }

        score >= self.solution_threshold
    }

    /// Run the mining process across multiple threads
    ///
    /// # Arguments
    /// * `miner` - An Arc-wrapped instance of the Miner struct
    pub fn run(miner: &Arc<Miner>) {
        for idx in 0..miner.num_threads {
            let miner_clone = miner.clone();

            tokio::spawn(async move {
                let mut nonce: Nonce64 = Nonce64::default();
                let mut neuron_data = NeuronData::default();
                let mut nonce_for_send: Vec<Nonce64> = Vec::new();

                loop {
                    log::debug!("[{}] Finding solution in Thread Id ({:?})", idx, thread::current().id());

                    if miner_clone.find_solution(&mut nonce, &mut neuron_data) {
                        miner_clone.score_counter.fetch_add(1, Ordering::Relaxed);
                        nonce_for_send.push(nonce);
                    }

                    if !nonce_for_send.is_empty() {
                        if let Ok(mut lock) = miner_clone.found_nonce.try_lock() {
                            lock.append(&mut nonce_for_send);
                        }
                    }

                    miner_clone.iteration_counter.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    }
}

/// Generate a random 64-bit number using the RDRAND instruction
///
/// # Returns
/// A 64-bit random number
fn generate_random_u64() -> u64 {
    let mut value: u64 = 0;
    unsafe {
        _rdrand64_step(&mut value);
    }
    value
}
