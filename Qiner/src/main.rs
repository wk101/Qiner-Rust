//! Qiner: High-performance cryptographic miner
//!
//! This binary initializes the mining environment, retrieves settings from `.env`,
//! spawns mining workers, and manages asynchronous TCP communication to report results.
// === Standard Library ===
use std::env;                         // For reading environment variables
use std::mem::{size_of, transmute};  // For low-level memory manipulation (used in mining/packet serialization)
use std::sync::Arc;                  // For thread-safe shared references (used across threads)

// === Tokio Async Runtime ===
use tokio;                           // Base Tokio crate (often used for macros or attribute)
use tokio::io::AsyncWriteExt;        // Async trait for writing to TCP streams
use tokio::net::TcpStream;           // Async TCP socket for peer communication
use tokio::runtime::Builder;         // Used to configure and build a custom Tokio runtime

// === Qiner Crate (Project-Specific Core Logic) ===
use qiner::converters::get_public_key_64_from_id;  // Converts node ID into a 64-byte public key
use qiner::miner::Miner;                            // Core mining logic implementation
use qiner::network::Packet;                         // Basic unit of network transmission

// === Lib Crate (Shared Utilities and Constants) ===
use lib::env_names::{                           // Constants for reading from env variables
    ENV_ID,
    ENV_NUMBER_OF_THREADS,
    ENV_SERVER_IP,
    ENV_SERVER_PORT,
};
use lib::random_seed::get_random_seed;          // Utility to generate a reproducible or random seed
use lib::solution_threshold::get_solution_threshold;  // Returns current difficulty or threshold
use lib::types::{Id, PublicKey64, STACK_SIZE};   // Core types used across mining and networking
use lib::types::network::protocols::BROADCAST_MESSAGE; // Protocol constant for broadcast messaging
use lib::version::get_version;                  // Returns client version for logging/handshake

/// Retrieve the number of threads from the environment variable.
///
/// # Returns
/// The number of threads as a `usize`.
/// Returns a default value of 4 if parsing fails.
fn get_number_of_threads() -> usize {
    env::var(ENV_NUMBER_OF_THREADS).unwrap_or_else(|_| "4".to_string()).parse::<usize>().unwrap_or(4)
}

/// Retrieve the server IP address from the environment variable.
///
/// # Returns
/// The server IP address as a `String`.
/// Returns an empty string if the environment variable is not set.
fn get_server_ip() -> String {
    env::var(ENV_SERVER_IP).unwrap_or_default()
}

/// Retrieve the server port from the environment variable.
///
/// # Returns
/// The server port as a `String`.
/// Returns an empty string if the environment variable is not set.
fn get_server_port() -> String {
    env::var(ENV_SERVER_PORT).unwrap_or_default()
}

/// Retrieve the ID from the environment variable.
///
/// # Returns
/// The ID as a `String`.
/// Returns an empty string if the environment variable is not set.
fn get_id() -> String {
    env::var(ENV_ID).unwrap_or_default()
}

#[tokio::main]
async fn main() {
    // Initialize dotenv
    dotenv::dotenv().ok();

    // Initialize the logger
    pretty_env_logger::init_timed();

    // Retrieve the number of threads
    let number_of_threads = get_number_of_threads() + 1;
    let stack_size = STACK_SIZE * number_of_threads;

    // Build the Tokio runtime with a specified number of worker threads and stack size
    Builder::new_multi_thread()
        .worker_threads(number_of_threads)
        .thread_stack_size(stack_size)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            async_main().await;
        });
}

/// Main asynchronous function that runs the mining process and TCP communication
async fn async_main() {
    // Retrieve environment variables and other configurations
    let number_of_threads = get_number_of_threads();
    let ip_raw = get_server_ip();
    let port_raw = get_server_port();
    let id_raw = get_id();
    let version = get_version();
    let random_seed = get_random_seed();
    let solution_threshold = get_solution_threshold();

    // Display retrieved information
    log::info!("Version: {:?}", version);
    log::info!("Random seed: {:?}", random_seed);
    log::info!("Solution threshold: {:?}", solution_threshold);
    log::info!("IP address: {ip_raw}");
    log::info!("Port: {port_raw}");
    log::info!("Id: {id_raw}");
    log::info!("Available cores: {}", num_cpus::get());
    log::info!("Number of threads: {}", number_of_threads);

    // Convert ID to a byte array
    let id = match id_raw.as_bytes().try_into() {
        Ok(id) => id,
        Err(_) => {
            log::error!("Invalid ID format!");
            return;
        }
    };

    // Retrieve the public key from the ID
    let mut public_key: PublicKey64 = Default::default();
    if !get_public_key_64_from_id(&id, &mut public_key) {
        log::error!("Invalid ID!");
        return;
    }

    // Initialize the miner with the public key and number of threads
    let arc_miner = Arc::new(Miner::new(public_key, number_of_threads));
    Miner::run(&arc_miner);

    // Display task for monitoring mining progress
    let sent_score_counter = Arc::new(tokio::sync::Mutex::new(0usize));

    // Launch the display information task
    let display_info_future = display_info_task(arc_miner.clone(), sent_score_counter.clone());

    // Launch the TCP client task to send solutions to the server
    let send_solution_future = send_solution_task(arc_miner.clone(), sent_score_counter.clone(), ip_raw, port_raw, public_key);

    // Run the display and solution sending tasks concurrently
    tokio::join!(
        display_info_future,
        send_solution_future
    );

    println!("End");
}

/// Asynchronous task to display mining progress information
///
/// # Arguments
/// * `arc_miner` - Shared reference to the Miner instance
/// * `sent_score_counter` - Shared counter for sent scores
///
/// # Returns
/// An async future
async fn display_info_task(arc_miner: Arc<Miner>, sent_score_counter: Arc<tokio::sync::Mutex<usize>>) -> impl std::future::Future<Output = ()> {
    let mut prev_iter_value: usize = 0;

    loop {
        let score = arc_miner.get_score();
        let sent_scores = *sent_score_counter.lock().await;
        let it_per_sec = arc_miner.get_iter_counter() - prev_iter_value;
        prev_iter_value = arc_miner.get_iter_counter();

        log::info!("{} scores | sent scores {} | {} it/s", score, sent_scores, it_per_sec);

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

/// Asynchronous task to send mining solutions to the server
///
/// # Arguments
/// * `arc_miner` - Shared reference to the Miner instance
/// * `sent_score_counter` - Shared counter for sent scores
/// * `ip_raw` - IP address of the server
/// * `port_raw` - Port of the server
/// * `public_key` - Public key used for mining
///
/// # Returns
/// An async future
async fn send_solution_task(
    arc_miner: Arc<Miner>,
    sent_score_counter: Arc<tokio::sync::Mutex<usize>>,
    ip_raw: String,
    port_raw: String,
    public_key: PublicKey64
) -> impl std::future::Future<Output = ()> {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        let is_nonce_exists = !arc_miner.found_nonce.lock().await.is_empty();

        if is_nonce_exists {
            let addr = format!("{ip_raw}:{port_raw}");

            log::info!("Connecting to {addr}");
            let mut stream_result = TcpStream::connect(addr).await;

            match stream_result.as_mut() {
                Err(err) => {
                    log::error!("Failed to connect: {:?}", err);
                }
                Ok(stream) => {
                    // Wait for the socket to be writable
                    if let Err(err) = stream.writable().await {
                        log::error!("Writable: {:?}", err);
                    } else {
                        // Grab data
                        let data_for_send = {
                            let found_nonce = arc_miner.found_nonce.lock().await;
                            found_nonce.iter().map(|nonce| {
                                let packet = Packet::new(&BROADCAST_MESSAGE, &public_key, nonce);
                                unsafe { transmute::<Packet, [u8; size_of::<Packet>()]>(packet) }
                            }).collect::<Vec<[u8; size_of::<Packet>()]>>().into_iter().flatten().collect::<Vec<u8>>()
                        };

                        let packet_num = data_for_send.len() / size_of::<Packet>();
                        log::info!("TCP: will be sent {packet_num} packets({} Bytes)", data_for_send.len());

                        // Send data
                        log::info!("TCP: send data...");
                        let write_result = stream.write_all(data_for_send.as_slice()).await;
                        if let Err(err) = write_result {
                            log::error!("Failed to send data: {:?}", err);
                        } else {
                            let mut lock = sent_score_counter.lock().await;
                            *lock += packet_num;
                        }

                        // Deleting nonce that have been sent
                        arc_miner.found_nonce.lock().await.drain(0..packet_num);
                    }
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
