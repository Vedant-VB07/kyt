use rayon::prelude::*;
use std::collections::HashSet;
use crate::models::*;
use crate::policy::validate;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;

static SYMBOLS: &[char] = &['!', '@', '#'];
static SEPARATORS: &[&str] = &["", "_", "-", "."];
static LEET_MAP: &[(char, char)] = &[
    ('a', '@'),
    ('o', '0'),
    ('e', '3'),
    ('i', '1'),
    ('s', '$'),
];

#[derive(Clone)]
struct Category {
    values: Vec<String>,
}

fn dedup_clean(mut v: Vec<String>) -> Vec<String> {
    v.retain(|s| !s.trim().is_empty());
    v = v.into_iter().map(|s| s.trim().to_string()).collect();
    v.sort();
    v.dedup();
    v
}

fn collect_categories(persona: &Persona) -> Vec<Category> {
    let mut categories = Vec::new();

    let identity = dedup_clean({
    let mut v = Vec::new();

    v.extend(persona.identity.full_name.split_whitespace().map(|s| s.to_string()));
    v.extend(persona.identity.nicknames.clone());
    v.extend(persona.identity.spouse.clone());
    v.extend(persona.identity.children.clone());
    v.extend(persona.identity.pets.clone());
    v.extend(persona.identity.maiden_names.clone());

    // 🔥 Inject mandatory inclusion strings as seeds
    v.extend(persona.policy.mandatory_include.clone());

    v
    });

    if !identity.is_empty() {
        categories.push(Category { values: identity });
    }

    let geography = dedup_clean({
        let mut v = Vec::new();
        v.extend(persona.geography.birth_city.clone());
        v.extend(persona.geography.current_city.clone());
        v.extend(persona.geography.streets.clone());
        v.extend(persona.geography.vacation_spots.clone());
        v
    });

    if !geography.is_empty() {
        categories.push(Category { values: geography });
    }

    let professional = dedup_clean({
        let mut v = Vec::new();
        v.extend(persona.professional.current_company.clone());
        v.extend(persona.professional.previous_employers.clone());
        v.extend(persona.professional.departments.clone());
        v.extend(persona.professional.projects.clone());
        v
    });

    if !professional.is_empty() {
        categories.push(Category { values: professional });
    }

    let personal = dedup_clean({
        let mut v = Vec::new();
        v.extend(persona.personal.sports_teams.clone());
        v.extend(persona.personal.bands.clone());
        v.extend(persona.personal.hobbies.clone());
        v.extend(persona.personal.cars.clone());
        v
    });

    if !personal.is_empty() {
        categories.push(Category { values: personal });
    }

    categories
}

fn derive_date_fragments(persona: &Persona) -> Vec<String> {
    let mut fragments = Vec::new();

    let mut all_dates = Vec::new();
    all_dates.extend(persona.chronology.birthdates.clone());
    all_dates.extend(persona.chronology.anniversaries.clone());
    all_dates.extend(persona.chronology.graduation_years.clone());
    all_dates.extend(persona.chronology.employment_start.clone());

    for date in all_dates {
        if date.len() >= 8 {
            fragments.push(date[0..2].to_string());
            fragments.push(date[2..4].to_string());
            fragments.push(date[0..4].to_string());
        }
        if date.len() >= 4 {
            fragments.push(date[date.len()-4..].to_string());
        }
        if date.len() >= 2 {
            fragments.push(date[date.len()-2..].to_string());
        }
    }

    fragments.sort();
    fragments.dedup();
    fragments
}

fn cross_categories(categories: &[Category], aggressive: bool) -> Vec<String> {
    let mut results = Vec::new();
    let n = categories.len();
    let max_depth = if aggressive { 4 } else { 3 };

    fn recurse(
        categories: &[Category],
        current: Vec<String>,
        depth: usize,
        max_depth: usize,
        results: &mut Vec<String>,
    ) {
        if depth > 0 {
            results.push(current.join(""));
        }

        if depth == max_depth {
            return;
        }

        for cat in categories {
            for val in &cat.values {
                let mut next = current.clone();
                next.push(val.clone());
                recurse(categories, next, depth + 1, max_depth, results);
            }
        }
    }

    for cat in categories {
        for val in &cat.values {
            recurse(categories, vec![val.clone()], 1, max_depth, &mut results);
        }
    }

    results
}

fn case_variants(word: &str, aggressive: bool) -> Vec<String> {
    let mut variants = vec![
        word.to_string(),
        word.to_lowercase(),
        word.to_uppercase(),
    ];

    if let Some(first) = word.chars().next() {
        variants.push(first.to_uppercase().collect::<String>() + &word[1..]);
    }

    if aggressive {
        variants.push(word.chars().rev().collect());
    }

    variants.sort();
    variants.dedup();
    variants
}

fn numeric_layer(word: &str, policy: &PasswordPolicy, aggressive: bool) -> Vec<String> {
    if !policy.require_numeric {
        return vec![word.to_string()];
    }

    let remaining = policy.max_length.saturating_sub(word.len());
    let mut variants = Vec::new();

    // In aggressive mode allow full remaining width
    // In normal mode limit to max 3 digits
    let max_digits = if aggressive {
        remaining
    } else {
        remaining.min(3)
    };

    for digits in 1..=max_digits {
        let max = 10usize.pow(digits as u32);

        for i in 0..max {
            variants.push(format!("{}{:0width$}", word, i, width = digits));
        }
    }

    variants
}

fn symbol_layer(word: &str, policy: &PasswordPolicy, aggressive: bool) -> Vec<String> {
    if !policy.require_symbol {
        return vec![word.to_string()];
    }

    let mut variants = Vec::new();

    for sym in SYMBOLS {
        variants.push(format!("{}{}", word, sym));
        variants.push(format!("{}{}", sym, word));

        if word.len() > 1 {
            variants.push(format!("{}{}{}", &word[0..1], sym, &word[1..]));
        }

        if aggressive && word.len() > 2 {
            variants.push(format!("{}{}{}", &word[..2], sym, &word[2..]));
        }
    }

    variants
}

fn leet_layer(word: &str, aggressive: bool) -> Vec<String> {
    let mut variants = vec![word.to_string()];

    for (from, to) in LEET_MAP {
        if word.contains(*from) {
            variants.push(word.replace(*from, &to.to_string()));

            if aggressive {
                variants.push(
                    word.replace(*from, &to.to_string())
                        .chars()
                        .rev()
                        .collect()
                );
            }
        }
    }

    variants
}

fn length_ok(word: &str, policy: &PasswordPolicy) -> bool {
    word.len() >= policy.min_length && word.len() <= policy.max_length
}

fn cross_categories_segments(categories: &[Category], aggressive: bool) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    let max_depth = if aggressive { 4 } else { 3 };

    fn recurse(
        categories: &[Category],
        current: Vec<String>,
        depth: usize,
        max_depth: usize,
        results: &mut Vec<Vec<String>>,
    ) {
        if depth > 0 {
            results.push(current.clone());
        }

        if depth == max_depth {
            return;
        }

        for cat in categories {
            for val in &cat.values {
                let mut next = current.clone();
                next.push(val.clone());
                recurse(categories, next, depth + 1, max_depth, results);
            }
        }
    }

    for cat in categories {
        for val in &cat.values {
            recurse(categories, vec![val.clone()], 1, max_depth, &mut results);
        }
    }

    results
}

fn segment_case_combinations(segments: &[String], aggressive: bool) -> Vec<String> {
    fn case_variants(word: &str) -> Vec<String> {
        let mut variants = vec![
            word.to_string(),
            word.to_lowercase(),
            word.to_uppercase(),
        ];

        if let Some(first) = word.chars().next() {
            variants.push(first.to_uppercase().collect::<String>() + &word[1..]);
        }

        variants.sort();
        variants.dedup();
        variants
    }

    fn combine(
        segments: &[String],
        index: usize,
        current: Vec<String>,
        results: &mut Vec<String>,
        aggressive: bool,
    ) {
        if index == segments.len() {
            results.push(current.join(""));
            return;
        }

        let variants = case_variants(&segments[index]);

        for var in variants {
            let mut next = current.clone();
            next.push(var);
            combine(segments, index + 1, next, results, aggressive);
        }

        if aggressive {
            let reversed = segments[index].chars().rev().collect::<String>();
            let mut next = current.clone();
            next.push(reversed);
            combine(segments, index + 1, next, results, aggressive);
        }
    }

    let mut results = Vec::new();
    combine(segments, 0, Vec::new(), &mut results, aggressive);
    results
}



pub fn generate(persona: &Persona, aggressive: bool) -> HashSet<String> {
    use indicatif::{ProgressBar, ProgressStyle};
    use std::sync::{Arc, atomic::{AtomicU64, AtomicBool, Ordering}};
    use std::thread;
    use std::time::{Duration, Instant};

    let categories = collect_categories(persona);
    let date_fragments = derive_date_fragments(persona);
    let policy = &persona.policy;

    let base_segments = cross_categories_segments(&categories, aggressive);

    let counter = Arc::new(AtomicU64::new(0));
    let running = Arc::new(AtomicBool::new(true));

    // 🔥 Start telemetry thread only in aggressive mode
    let monitor = if aggressive {
        let counter_clone = counter.clone();
        let running_clone = running.clone();

        Some(thread::spawn(move || {
            let start = Instant::now();
            let pb = ProgressBar::new_spinner();

            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner} {msg}")
                    .unwrap()
            );

            pb.enable_steady_tick(Duration::from_millis(100));

            while running_clone.load(Ordering::Relaxed) {
                let generated = counter_clone.load(Ordering::Relaxed);
                let elapsed = start.elapsed().as_secs_f64();
                let rate = if elapsed > 0.0 {
                    (generated as f64 / elapsed) as u64
                } else {
                    0
                };

                pb.set_message(format!(
                    "Generated: {} | {} passwords/sec | Elapsed: {}",
                    generated,
                    rate,
                    format_duration(start.elapsed())
                ));

                thread::sleep(Duration::from_millis(500));
            }

            pb.finish_with_message("Generation complete.");
        }))
    } else {
        None
    };

    // 🔥 Actual generation
    let results: HashSet<String> = base_segments
        .par_iter()
        .flat_map(|segments| {

            let mut expanded = Vec::new();

            expanded.push(segments.clone());

            let mut reversed = segments.clone();
            reversed.reverse();
            expanded.push(reversed);

            for fragment in &date_fragments {
                let mut with_suffix = segments.clone();
                with_suffix.push(fragment.clone());
                expanded.push(with_suffix);

                let mut with_prefix = vec![fragment.clone()];
                with_prefix.extend(segments.clone());
                expanded.push(with_prefix);
            }

            expanded
        })
        .flat_map(|segments| {

            segment_case_combinations(&segments, aggressive)
                .into_iter()
                .flat_map(|joined| {

                    if joined.len() > policy.max_length {
                        return Vec::new();
                    }

                    numeric_layer(&joined, policy, aggressive)
                        .into_iter()
                        .flat_map(|n| symbol_layer(&n, policy, aggressive))
                        .flat_map(|s| leet_layer(&s, aggressive))
                        .filter(|final_word| length_ok(final_word, policy))
                        .map(|final_word| {
                            counter.fetch_add(1, Ordering::Relaxed);
                            final_word
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<String>>()
        })
        .filter(|candidate| validate(candidate, policy))
        .collect();

    // 🔥 Stop telemetry
    running.store(false, Ordering::Relaxed);

    if let Some(handle) = monitor {
        let _ = handle.join();
    }

    results
}

fn format_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}