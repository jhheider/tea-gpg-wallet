mod bpb;
mod gpg;
mod utils;

use crate::utils::{
    SigningResult, filter_hex_string, hex_color, require_private_key, wei_to_eth_auto,
};
use anyhow::{Context, Result, anyhow};
use clap::{Arg, ArgAction::SetTrue, ArgGroup, ArgMatches, command};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use libtea_gpg_wallet::{
    deployer::{self, get_key_id_balance, predict_address, send_to_gpg_key},
    utils::{decimal_to_wei_precise, get_rpc_url},
    wallet::{SigningData, get_signable_hash, sweep_gpg_key},
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let key_arguments = [
        Arg::new("key_id").help("The GPG key ID to map to wallet address"),
        Arg::new("bpb")
            .long("bpb")
            .short('b')
            .help("Use bpb to get the key ID")
            .action(SetTrue),
        Arg::new("gpg")
            .long("gpg")
            .short('g')
            .help("Use gpg to get the key ID for an email address"),
    ];
    let m = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(command!("config").about("Prints default configuration."))
        .subcommand(
            command!("find")
                .about("Finds the GPG wallet address for a given key ID")
                .arg_required_else_help(true)
                .args(&key_arguments)
                .group(
                    ArgGroup::new("key_id_group")
                        .args(["key_id", "bpb", "gpg"])
                        .required(true),
                ),
        )
        .subcommand(
            command!("deploy")
                .about("Deploys the GPG wallet contract for a given key ID")
                .arg_required_else_help(true)
                .args(&key_arguments)
                .group(
                    ArgGroup::new("key_id_group")
                        .args(["key_id", "bpb", "gpg"])
                        .required(true),
                ),
        )
        .subcommand(
            command!("send")
                .about("Sends TEA to the GPG wallet for a given key ID, deploying the wallet if necessary,\n  (reads the private key from the PRIVATE_KEY environment variable)")
                .arg_required_else_help(true)
                .arg(Arg::new("amount")
                    .help("Amount of TEA to send")
                    .required(true))
                .args(&key_arguments)
                .group(
                    ArgGroup::new("key_id_group")
                        .args(["key_id", "bpb", "gpg"])
                        .required(true),
                )
        )
        .subcommand(
            command!("sweep")
                .about("Sweeps the GPG wallet for a given key ID,\n  (reads the private key from the PRIVATE_KEY environment variable)")
                .arg_required_else_help(true)
                .arg(Arg::new("destination")
                    .help("Address to sweep to")
                    .required(true))
                .args(&key_arguments[1..]) // Exclude "key_id" since we can't sweep without a private key
                .group(
                    ArgGroup::new("key_id_group")
                        .args(["bpb", "gpg"])
                        .required(true),
                )
        )
        .get_matches();

    match m.subcommand() {
        Some(("config", _)) => {
            println!("Default configuration:\n");
            println!("{}", "RPC URL:".blue().bold());
            println!("  {}", get_rpc_url()?.to_string().green());
            println!("{}", "Deployer address:".blue().bold());
            println!(
                "  {}",
                hex_color(deployer::get_contract_address()?.to_string(), 4)?
            );
        }
        Some(("find", sub_m)) => {
            let key_id = get_key_id(sub_m).await?;
            let prediction = predict_address(&key_id).await?;
            let deployed = if prediction.isDeployed {
                "deployed".green().bold()
            } else {
                "not deployed".red().bold()
            };
            println!(
                "{} {}:",
                "Predicted address for key ID".blue().bold(),
                hex_color(&key_id, 4)?
            );
            println!(
                "\t{} ({deployed})",
                hex_color(prediction.walletAddress.to_string(), 4)?
            );
        }
        Some(("deploy", sub_m)) => {
            let private_key = require_private_key()?;
            let key_id = get_key_id(sub_m).await?;
            let prediction = deployer::ensure_deployed(&key_id, &private_key).await?;
            println!(
                "{} {}:",
                "Deployed address for key ID".blue().bold(),
                hex_color(&key_id, 4)?
            );
            println!("\t{}", hex_color(prediction.walletAddress.to_string(), 4)?);
        }
        Some(("send", sub_m)) => {
            let private_key = require_private_key()?;
            let key_id = get_key_id(sub_m).await?;
            let amount_str = sub_m
                .get_one::<String>("amount")
                .context("Amount not provided")?;
            let amount = decimal_to_wei_precise(amount_str)?;
            let balance = get_key_id_balance(&key_id).await?;
            println!(
                "{} {}: {}",
                "Balance for key ID".blue().bold(),
                hex_color(&key_id, 4)?,
                wei_to_eth_auto(balance).green()
            );
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(100));
            pb.set_style(
                ProgressStyle::with_template("{spinner:.green} {msg}")
                    .unwrap()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
            );
            pb.set_message(format!(
                "Sending {} to key ID {}",
                wei_to_eth_auto(amount).green(),
                hex_color(&key_id, 4)?
            ));
            let new_balance = send_to_gpg_key(&key_id, amount, &private_key).await?;
            pb.finish_with_message("Send completed".green().to_string());
            println!(
                "{} {}: {}",
                "New balance for key ID".blue().bold(),
                hex_color(&key_id, 4)?,
                wei_to_eth_auto(new_balance).green()
            );
        }
        Some(("sweep", sub_m)) => {
            let private_key = require_private_key()?;
            let key_id = get_key_id(sub_m).await?;
            let to_address = sub_m
                .get_one::<String>("destination")
                .context("Destination address not provided")?;
            let to_address = filter_hex_string(to_address.strip_prefix("0x").unwrap_or(to_address));
            let balance = get_key_id_balance(&key_id).await?;
            println!(
                "{} {}: {}",
                "Balance for key ID".blue().bold(),
                hex_color(&key_id, 4)?,
                wei_to_eth_auto(balance).green()
            );
            if balance.is_zero() {
                eprintln!("No balance to sweep for key ID {key_id}");
                return Ok(());
            }
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(100));
            pb.set_style(
                ProgressStyle::with_template("{spinner:.green} {msg}")
                    .unwrap()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
            );
            pb.set_message(format!(
                "Sweeping {} from key ID {} to 0x{}",
                wei_to_eth_auto(balance).green(),
                hex_color(&key_id, 4)?,
                to_address.green()
            ));
            let signing_data = get_signable_hash(&key_id, &to_address)
                .await
                .context("Failed to get signable hash")?;
            let signature = sign_with_key(sub_m, &signing_data).await?;
            let tx = sweep_gpg_key(
                &key_id,
                &to_address,
                signing_data.deadline,
                &signature.public_key,
                &signature.signature,
                &private_key,
            )
            .await
            .context("Failed to sweep GPG wallet")?;
            pb.finish_with_message("Sweep completed".green().to_string());
            println!(
                "{} {}: {}",
                "Sweep transaction hash for key ID".blue().bold(),
                hex_color(&key_id, 4)?,
                hex_color(tx.to_string(), 4)?
            );
            let new_balance = get_key_id_balance(&key_id).await?;
            println!(
                "{} {}: {}",
                "New balance for key ID".blue().bold(),
                hex_color(&key_id, 4)?,
                wei_to_eth_auto(new_balance).green()
            );
        }

        _ => unreachable!(),
    }
    Ok(())
}

async fn get_key_id(sub_m: &ArgMatches) -> Result<String> {
    if sub_m.try_get_one::<String>("key_id").is_ok() && sub_m.get_one::<String>("key_id").is_some()
    {
        sub_m
            .get_one::<String>("key_id")
            .map(|s| filter_hex_string(s))
            .context("Key ID not valid")
    } else if sub_m.get_flag("bpb") {
        bpb::get_key_id()
            .await
            .context("Failed to get key ID from bpb")
    } else if sub_m.get_one::<String>("gpg").is_some() {
        let email = sub_m
            .get_one::<String>("gpg")
            .context("Email address not provided")?;
        gpg::get_key_id(email)
            .await
            .context("Failed to get key ID from gpg")
    } else {
        Err(anyhow!("No key ID provided"))
    }
}

async fn sign_with_key(sub_m: &ArgMatches, blob: &SigningData) -> Result<SigningResult> {
    if sub_m.get_flag("bpb") {
        return bpb::sign_blob(blob)
            .await
            .context("Failed to sign blob with bpb");
    } else if sub_m.get_one::<String>("gpg").is_some() {
        let key_id = get_key_id(sub_m).await?;
        return gpg::sign_blob(blob, &key_id)
            .await
            .context("Failed to sign blob with gpg");
    }
    Err(anyhow!("No signing method provided"))
}
