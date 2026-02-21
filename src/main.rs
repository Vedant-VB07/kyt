mod interview;
mod models;
mod mutation;
mod policy;
mod writer;
mod bruteforce;

use clap::Parser;
use std::fs;
use models::Persona;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    json: Option<String>,

    #[arg(long)]
    bruteforce: bool,

    #[arg(long)]
    resume: Option<u128>,

    /// Enable aggressive CTF mutation mode
    #[arg(long)]
    aggressive: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // ================================
    // Load Persona (JSON or Interactive)
    // ================================
    let persona: Persona = if let Some(file) = args.json {
        let data = fs::read_to_string(file)?;
        serde_json::from_str(&data)?
    } else {
        interview::run_interview()
    };

    // ================================
    // BRUTE FORCE MODE (Streaming)
    // ================================
    if args.bruteforce {
        println!("[*] Starting streaming brute-force engine...");
        bruteforce::run_bruteforce(
            &persona.policy,
            &persona.output_file,
            args.resume,
        )?;
        println!("[✓] Bruteforce generation complete.");
        return Ok(());
    }

    // =================================
    // PERSONA MUTATION MODE
    // =================================
    if args.aggressive {
        println!("[*] Generating persona-based permutations (AGGRESSIVE CTF MODE)...");
    } else {
        println!("[*] Generating persona-based permutations...");
    }

    // 🔥 No spinner here anymore
    let results = mutation::generate(&persona, args.aggressive);

    println!("[*] Writing {} unique entries...", results.len());
    writer::write_to_file(&persona.output_file, &results)?;

    println!("[✓] Output saved to {}", persona.output_file);

    Ok(())
}