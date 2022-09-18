use crate::utils::{clear_wallets_vault, DEFAULT_WALLETS_VAULT_PATH};
use anyhow::{bail, Result};
use fuels::signers::wallet::Wallet;
use std::path::PathBuf;

pub(crate) fn import_wallet(path: Option<String>) -> Result<()> {
    let wallet_path = match &path {
        Some(path) => {
            // If the provided path exists but used clear it
            clear_wallets_vault(&PathBuf::from(&path))?;
            // If the provided path does not exists create it
            std::fs::create_dir_all(path)?;
            PathBuf::from(path)
        }
        None => {
            let mut path = home::home_dir().unwrap();
            path.push(DEFAULT_WALLETS_VAULT_PATH);
            // If the default vault path exists but used clear it
            clear_wallets_vault(&path)?;
            // If the default vault path does not exists create it
            std::fs::create_dir_all(&path)?;
            path
        }
    };

    let mnemonic = rpassword::prompt_password("Please enter your mnemonic phrase: ")?;
    // Check users's phrase by trying to create a wallet from it
    if let Err(e) = Wallet::new_from_mnemonic_phrase(&mnemonic, None) {
        bail!("Please check your phrase: {e}");
    }
    // Encyrpt and store it
    let mnemonic_bytes: Vec<u8> = mnemonic.bytes().collect();
    let password = rpassword::prompt_password("Please enter a password to encrypt the phrase: ")?;
    let confirmation = rpassword::prompt_password("Please confirm your password: ")?;

    if password != confirmation {
        bail!("Passwords do not match -- try again!");
    }
    eth_keystore::encrypt_key(
        &wallet_path,
        &mut rand::thread_rng(),
        mnemonic_bytes,
        &password,
        Some(".wallet"),
    )?;
    Ok(())
}