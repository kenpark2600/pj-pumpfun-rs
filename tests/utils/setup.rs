use std::{
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard, OnceLock},
};

use anchor_client::Cluster;
use pumpfun::PumpFun;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair},
};

// Load the default keypair with error handling
fn load_default_keypair() -> Result<Keypair, String> {
    let home_path = std::env::var_os("HOME").ok_or("HOME environment variable not set")?;
    let default_keypair_path = PathBuf::from(home_path).join(".config/solana/id.json");
    read_keypair_file(&default_keypair_path).map_err(|e| {
        format!(
            "Failed to read keypair from {}: {}",
            default_keypair_path.display(),
            e
        )
    })
}

// Global state with OnceLock
static DEFAULT_KEYPAIR: OnceLock<Keypair> = OnceLock::new();
static PAYER: OnceLock<Arc<Keypair>> = OnceLock::new();
static MINT: OnceLock<Arc<Keypair>> = OnceLock::new();
static CLIENT: OnceLock<Mutex<PumpFun>> = OnceLock::new();

// Initialize global state
fn initialize_globals() {
    // Load and set the default keypair
    let keypair = load_default_keypair().expect("Failed to load default keypair");
    DEFAULT_KEYPAIR
        .set(keypair)
        .expect("DEFAULT_KEYPAIR already set");

    // Set payer and mint keypairs
    PAYER
        .set(Arc::new(DEFAULT_KEYPAIR.get().unwrap().insecure_clone()))
        .expect("PAYER already set");
    MINT.set(Arc::new(Keypair::from_base58_string(
        "2Cc5p7aNW8jsTbDxmPMfguDxwrDFWUu1s93gABJYTUJ2xGLF9w2EpPCW1CGFvKYAWzuHXh5fLhrmroHjd8LwBQxj",
    )))
    .expect("MINT already set");

    // Initialize the PumpFun client with configurable cluster URLs
    CLIENT
        .set(Mutex::new(PumpFun::new(
            Cluster::Custom(
                "http://127.0.0.1:8899".to_string(),
                "ws://127.0.0.1:8900".to_string(),
            ),
            PAYER.get().unwrap().clone(),
            Some(CommitmentConfig::finalized()),
            None,
        )))
        .unwrap_or_else(|_| panic!("CLIENT already set"));
}

pub struct TestContext {
    pub payer: Arc<Keypair>,
    pub mint: Arc<Keypair>,
    pub client: MutexGuard<'static, PumpFun>,
}

impl Default for TestContext {
    fn default() -> Self {
        // Ensure globals are initialized
        if DEFAULT_KEYPAIR.get().is_none() {
            initialize_globals();
        }

        Self {
            payer: PAYER.get().unwrap().clone(),
            mint: MINT.get().unwrap().clone(),
            client: CLIENT
                .get()
                .unwrap()
                .lock()
                .expect("Failed to lock CLIENT mutex"),
        }
    }
}
