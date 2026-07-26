use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "provider-trace", about = "Self-attested Soroban provider uptime/latency tool")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Fetch a node's /metrics endpoint and parse uptime/latency for submission
    FetchMetrics {
        /// URL of the node's /metrics endpoint (mutually exclusive with --file)
        #[arg(long, required_unless_present = "file")]
        metrics_url: Option<String>,

        /// Path to a local metrics file (mutually exclusive with --metrics-url)
        #[arg(long, required_unless_present = "metrics_url")]
        file: Option<String>,

        /// Provider ID as hex (32 bytes)
        #[arg(long)]
        provider_id: String,

        /// Period start time (Unix seconds)
        #[arg(long)]
        period_start: u64,

        /// Period end time (Unix seconds)
        #[arg(long)]
        period_end: u64,
    },
    /// Query a provider's attestation history from the contract
    Query {
        /// Soroban RPC endpoint
        #[arg(long)]
        rpc_url: String,

        /// Contract ID hosting the attestation contract
        #[arg(long)]
        contract_id: String,

        /// Provider ID as hex (32 bytes)
        #[arg(long)]
        provider_id: String,
    },
}

fn parse_prometheus_text(text: &str) -> HashMap<String, f64> {
    let mut metrics = HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, val_str)) = line.split_once(' ') {
            if let Ok(val) = val_str.parse::<f64>() {
                metrics.insert(key.to_string(), val);
            }
        }
    }
    metrics
}

fn parse_provider_metrics(text: &str) -> Result<(u32, u32)> {
    let metrics = parse_prometheus_text(text);

    // Try to find Soroban-specific metrics. Common patterns:
    // - soroban_uptime_basis_points (if the node exposes it directly)
    // - Or compute from soroban_uptime_seconds / soroban_total_seconds
    // Fall back to simple patterns for the sample metrics format.

    let uptime_bp = if let Some(v) = metrics.get("soroban_uptime_basis_points") {
        *v as u32
    } else if let (Some(up), Some(total)) =
        (metrics.get("soroban_uptime_seconds"), metrics.get("soroban_total_seconds"))
    {
        if *total > 0.0 {
            ((up / total) * 10000.0) as u32
        } else {
            0
        }
    } else if let Some(v) = metrics.get("uptime_percent_basis_points") {
        *v as u32
    } else {
        return Err(anyhow!(
            "could not find uptime metric in /metrics output. \
             Expected 'soroban_uptime_basis_points' or \
             'soroban_uptime_seconds' + 'soroban_total_seconds'. \
             \n\nNote: These values are self-reported by the node operator. \
             This tool does not verify their accuracy."
        ));
    };

    let latency_ms = if let Some(v) = metrics.get("soroban_avg_latency_ms") {
        *v as u32
    } else if let Some(v) = metrics.get("avg_latency_ms") {
        *v as u32
    } else {
        return Err(anyhow!(
            "could not find latency metric in /metrics output. \
             Expected 'soroban_avg_latency_ms'. \
             \n\nNote: These values are self-reported by the node operator. \
             This tool does not verify their accuracy."
        ));
    };

    if uptime_bp > 10000 {
        return Err(anyhow!(
            "parsed uptime_percent {} exceeds maximum 10000 (100%)",
            uptime_bp
        ));
    }

    Ok((uptime_bp, latency_ms))
}

#[derive(Serialize)]
struct RpcRequest<T: Serialize> {
    jsonrpc: String,
    id: u64,
    method: String,
    params: T,
}

#[derive(Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcError>,
}

#[derive(Deserialize)]
struct RpcError {
    message: String,
}

async fn query_history(rpc_url: &str, contract_id: &str, provider_id: &str) -> Result<()> {
    // Simulate a Soroban RPC call to get_provider_history.
    // In practice this would use soroban-spec or soroban-rpc types.
    let hex_provider_id = format!("{:0>64}", provider_id.trim_start_matches("0x"));

    let request = RpcRequest {
        jsonrpc: "2.0".into(),
        id: 1,
        method: "getContractData".into(),
        params: serde_json::json!({
            "contractId": contract_id,
            "key": {
                "symbol": "History",
                "args": [{
                    "bytes": hex_provider_id
                }]
            },
            "keyType": "vec"
        }),
    };

    let client = reqwest::Client::new();
    let resp: RpcResponse<serde_json::Value> = client
        .post(rpc_url)
        .json(&request)
        .send()
        .await
        .context("failed to send RPC request")?
        .json()
        .await
        .context("failed to parse RPC response")?;

    if let Some(err) = resp.error {
        eprintln!("RPC error: {}", err.message);
        return Ok(());
    }

    println!("\n--- Attestation History ---");
    println!(
        "DISCLAIMER: All attestation values shown below are self-reported \
         by the provider's registered operator. This contract does not verify \
         the accuracy or truthfulness of these claims. They should not be \
         treated as proof of reliability."
    );
    match resp.result {
        Some(result) => println!("Raw result: {}", serde_json::to_string_pretty(&result)?),
        None => println!("No attestations found for provider {}", provider_id),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::FetchMetrics {
            metrics_url,
            file,
            provider_id,
            period_start,
            period_end,
        } => {
            let text = if let Some(path) = file {
                std::fs::read_to_string(&path)
                    .context("failed to read metrics file")?
            } else if let Some(url) = metrics_url {
                println!("Fetching metrics from: {}", url);
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(15))
                    .build()?;
                client
                    .get(&url)
                    .send()
                    .await
                    .context("failed to fetch /metrics endpoint")?
                    .text()
                    .await
                    .context("failed to read response body")?
            } else {
                anyhow::bail!("either --metrics-url or --file is required");
            };

            let (uptime_bp, latency_ms) = parse_provider_metrics(&text)?;

            println!("\nParsed attestation values (self-reported, unverified):");
            println!("  Provider ID:     {}", provider_id);
            println!("  Period start:    {}", period_start);
            println!("  Period end:      {}", period_end);
            println!("  Uptime (bp):     {} ({}%)", uptime_bp, uptime_bp as f64 / 100.0);
            println!("  Avg latency ms:  {}", latency_ms);
            println!(
                "\nTo submit this attestation on-chain, use the provider-trace contract's \
                 submit_attestation function with your operator account.\n\
                 \nDISCLAIMER: These values are self-reported by the node at the /metrics \
                 endpoint. provider-trace does not verify the accuracy of any submitted data."
            );
        }
        Command::Query {
            rpc_url,
            contract_id,
            provider_id,
        } => {
            query_history(&rpc_url, &contract_id, &provider_id).await?;
        }
    }

    Ok(())
}
