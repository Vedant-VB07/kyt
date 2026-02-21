use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use crate::models::PasswordPolicy;
use crate::policy::validate;

fn build_charset(policy: &PasswordPolicy) -> Vec<char> {
    let mut charset = Vec::new();

    if policy.require_lower {
        charset.extend('a'..='z');
    }

    if policy.require_upper {
        charset.extend('A'..='Z');
    }

    if policy.require_numeric {
        charset.extend('0'..='9');
    }

    if policy.require_symbol {
        charset.extend(vec!['!', '@', '#', '$', '%', '&']);
    }

    // If nothing required, default to full printable set
    if charset.is_empty() {
        charset.extend('a'..='z');
        charset.extend('A'..='Z');
        charset.extend('0'..='9');
    }

    charset
}

fn index_to_candidate(mut index: u128, length: usize, charset: &[char]) -> String {
    let base = charset.len() as u128;
    let mut buffer = vec![' '; length];

    for i in (0..length).rev() {
        buffer[i] = charset[(index % base) as usize];
        index /= base;
    }

    buffer.iter().collect()
}

pub fn run_bruteforce(
    policy: &PasswordPolicy,
    output_file: &str,
    resume_from: Option<u128>,
) -> std::io::Result<()> {

    let charset = build_charset(policy);
    let charset_len = charset.len() as u128;

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)?;

    let writer = Arc::new(Mutex::new(BufWriter::new(file)));

    for length in policy.min_length..=policy.max_length {

        let total = charset_len.pow(length as u32);

        println!("[*] Length {} → {} combinations", length, total);

        let start = resume_from.unwrap_or(0);

        (start..total).into_par_iter().for_each(|i| {

            let candidate = index_to_candidate(i, length, &charset);

            if validate(&candidate, policy) {
                let mut locked = writer.lock().unwrap();
                writeln!(locked, "{}", candidate).unwrap();
            }
        });
    }

    Ok(())
}