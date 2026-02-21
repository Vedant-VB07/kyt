use rayon::prelude::*;
use std::collections::HashSet;
use crate::models::*;
use crate::policy::validate;

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
            fragments.push(date[0..2].to_string());  // day
            fragments.push(date[2..4].to_string());  // month
            fragments.push(date[0..4].to_string());  // ddmm
        }
        if date.len() >= 4 {
            fragments.push(date[date.len()-4..].to_string()); // yyyy
        }
        if date.len() >= 2 {
            fragments.push(date[date.len()-2..].to_string()); // yy
        }
    }

    fragments.sort();
    fragments.dedup();
    fragments
}

fn cross_categories(categories: &[Category]) -> Vec<String> {
    let mut results = Vec::new();
    let n = categories.len();

    // Depth 1
    for cat in categories {
        for val in &cat.values {
            results.push(val.clone());
        }
    }

    // Depth 2
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }

            for a in &categories[i].values {
                for b in &categories[j].values {
                    for sep in SEPARATORS {
                        results.push(format!("{}{}{}", a, sep, b));
                    }
                }
            }
        }
    }

    // Depth 3
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i == j || j == k || i == k { continue; }

                for a in &categories[i].values {
                    for b in &categories[j].values {
                        for c in &categories[k].values {
                            results.push(format!("{}{}{}", a, b, c));
                        }
                    }
                }
            }
        }
    }

    results
}

fn case_variants(word: &str) -> Vec<String> {
    let mut variants = vec![word.to_string()];
    variants.push(word.to_lowercase());
    variants.push(word.to_uppercase());

    if let Some(first) = word.chars().next() {
        variants.push(first.to_uppercase().collect::<String>() + &word[1..]);
    }

    variants.sort();
    variants.dedup();
    variants
}

fn reverse_variant(word: &str) -> String {
    word.chars().rev().collect()
}

fn numeric_layer(word: &str, policy: &PasswordPolicy) -> Vec<String> {
    if !policy.require_numeric {
        return vec![word.to_string()];
    }

    let mut variants = Vec::new();

    for i in 0..1000 { // 000–999
        variants.push(format!("{}{:03}", word, i));
    }

    variants
}

fn symbol_layer(word: &str, policy: &PasswordPolicy) -> Vec<String> {
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
    }

    variants
}

fn leet_layer(word: &str) -> Vec<String> {
    let mut variants = vec![word.to_string()];

    for (from, to) in LEET_MAP {
        if word.contains(*from) {
            variants.push(word.replace(*from, &to.to_string()));
        }
    }

    variants
}

fn length_ok(word: &str, policy: &PasswordPolicy) -> bool {
    word.len() >= policy.min_length && word.len() <= policy.max_length
}

pub fn generate(persona: &Persona) -> HashSet<String> {
    let categories = collect_categories(persona);
    let date_fragments = derive_date_fragments(persona);
    let policy = &persona.policy;

    let base_patterns = cross_categories(&categories);

    base_patterns.par_iter()
        .flat_map(|pattern| {

            let mut expanded = Vec::new();

            expanded.push(pattern.clone());
            expanded.push(reverse_variant(pattern));

            for fragment in &date_fragments {
                expanded.push(format!("{}{}", pattern, fragment));
                expanded.push(format!("{}{}", fragment, pattern));
            }

            expanded
        })
        .flat_map(|candidate| {

            if candidate.len() > policy.max_length {
                return Vec::new();
            }

            case_variants(&candidate)
                .into_iter()
                .flat_map(|c| numeric_layer(&c, policy))
                .flat_map(|n| symbol_layer(&n, policy))
                .flat_map(|s| leet_layer(&s))
                .filter(|final_word| length_ok(final_word, policy))
                .collect::<Vec<String>>()
        })
        .filter(|candidate| validate(candidate, policy))
        .collect()
}