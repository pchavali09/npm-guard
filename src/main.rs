use std::env;
use std::process::{Command, exit};
use std::time::Duration;
use chrono::{DateTime, Utc};
use colored::*;
use reqwest::blocking::Client;
use serde::Deserialize;

// --- CONFIGURATION ---
const REGISTRY_URL: &str = "https://registry.npmjs.org";
const SAFE_AGE_DAYS: i64 = 30;   // 🟢 established
const SQUAT_AGE_DAYS: i64 = 3;   // 🔴 toddler/squatter

// --- DATA STRUCTURES ---
#[derive(Deserialize, Debug)]
struct PackageMetadata {
    time: PackageTime,
}

#[derive(Deserialize, Debug)]
struct PackageTime {
    created: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // 1. INTELLIGENT INTERCEPTION
    // We strictly look for "install", "i", or "add"
    if args.len() > 1 && (args[1] == "install" || args[1] == "i" || args[1] == "add") {
        // Extract package names (ignore flags like --save-dev)
        let packages: Vec<&String> = args.iter()
            .skip(2) 
            .filter(|arg| !arg.starts_with("-")) 
            .collect();

        if !packages.is_empty() {
            println!("{}", "🚦 Vibe Check: Scanning registry...".yellow());
            scan_packages(packages);
        }
    }

    // 2. EXECUTE REAL NPM
    // If we haven't killed the process yet, we allow npm to run.
    let status = Command::new("npm")
        .args(&args[1..])
        .status();

    match status {
        Ok(s) => exit(s.code().unwrap_or(0)),
        Err(_) => {
            eprintln!("❌ Failed to execute npm. Is it in your PATH?");
            exit(1);
        }
    }
}

fn scan_packages(packages: Vec<&String>) {
    let client = Client::builder()
        .timeout(Duration::from_secs(2)) // 2s Timeout = Fail Fast
        .user_agent("npm-guard-cli/0.1")
        .build()
        .unwrap();

    let mut blocked = false;

    for pkg_name in packages {
        let url = format!("{}/{}", REGISTRY_URL, pkg_name);
        
        match client.get(&url).send() {
            Ok(resp) => {
                if resp.status() == 404 {
                    // --- 🔴 RULE 1: THE PHANTOM (404) ---
                    println!("\n{}", "👻 PHANTOM DETECTED".on_red().white().bold());
                    println!("Package '{}' does not exist.", pkg_name.red().bold());
                    println!("This is likely an AI hallucination. Do NOT install it.");
                    blocked = true;
                } else if resp.status().is_success() {
                    // --- ANALYZE METADATA ---
                    if let Ok(meta) = resp.json::<PackageMetadata>() {
                        check_age(pkg_name, &meta, &mut blocked);
                    }
                }
            },
            Err(_) => {
                // Network Error? -> 🟡 FAIL OPEN (Warn but allow)
                println!("⚠️  Check failed for '{}'. Network issue?", pkg_name.dimmed());
            }
        }
    }

    // 3. THE INTERVENTION
    if blocked {
        println!("\n{}", "🛑 SECURITY STOP".on_red().white().bold());
        println!("Potential malware or hallucination detected.");
        println!("Proceed anyway? (y/N)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("{}", "Aborted.".red());
            exit(1); // 💀 KILL SWITCH
        }
    }
}

fn check_age(pkg_name: &str, meta: &PackageMetadata, blocked: &mut bool) {
    let created_time = DateTime::parse_from_rfc3339(&meta.time.created);
    
    match created_time {
        Ok(date) => {
            let age = Utc::now().signed_duration_since(date).num_days();
            
            if age < SQUAT_AGE_DAYS {
                // --- 🔴 RULE 2: THE SQUATTER (< 3 Days) ---
                println!("\n{}", "👶 NEW PACKAGE ALERT".on_red().white().bold());
                println!("'{}' is only {} days old.", pkg_name.bold(), age);
                *blocked = true;
            } else if age < SAFE_AGE_DAYS {
                // --- 🟡 RULE 3: THE CAUTION (< 30 Days) ---
                println!("🟡 Caution: '{}' is fresh ({} days old).", pkg_name, age);
            } else {
                // --- 🟢 RULE 4: THE VETERAN (> 30 Days) ---
                println!("✅ {} is established ({} days old).", pkg_name.green(), age);
            }
        },
        Err(_) => println!("⚠️  Date parse error for {}", pkg_name),
    }
}