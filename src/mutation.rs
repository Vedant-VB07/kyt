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
    name: &'static str,
    values: Vec<String>,
}

fn dedup_clean(mut v: Vec<String>) -> Vec<String> {
    v.retain(|s| !s.trim().is_empty());
    v = v.into_iter().map(|s| s.trim().to_string()).collect();
    v.sort();
    v.dedup();
    v
}

fn collect_dynamic_categories(persona: &Persona) -> Vec<Category> {
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
        categories.push(Category { name: "identity", values: identity });
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
        categories.push(Category { name: "geography", values: geography });
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
        categories.push(Category { name: "professional", values: professional });
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
        categories.push(Category { name: "personal", values: personal });
    }

    categories
}

fn derive_all_year_fragments(persona: &Persona) -> Vec<String> {
    let mut years = Vec::new();

    let mut all_dates = Vec::new();
    all_dates.extend(persona.chronology.birthdates.clone());
    all_dates.extend(persona.chronology.anniversaries.clone());
    all_dates.extend(persona.chronology.graduation_years.clone());
    all_dates.extend(persona.chronology.employment_start.clone());

    for date in all_dates {
        if date.len() >= 4 {
            years.push(date[date.len()-4..].to_string());
        }
        if date.len() >= 2 {
            years.push(date[date.len()-2..].to_string());
        }
    }

    years.sort();
    years.dedup();
    years
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

fn enforce_case(mut word: String, policy: &PasswordPolicy) -> String {
    if policy.require_lower && !word.chars().any(|c| c.is_lowercase()) {
        word = word.to_lowercase();
    }

    if policy.require_upper && !word.chars().any(|c| c.is_uppercase()) {
        if let Some(first) = word.chars().next() {
            word = first.to_uppercase().collect::<String>() + &word[1..];
        }
    }

    word
}

fn numeric_layer(word: &str, policy: &PasswordPolicy) -> Vec<String> {
    if !policy.require_numeric {
        return vec![word.to_string()];
    }

    let mut variants = Vec::new();

    for i in 0..100 {
        variants.push(format!("{}{:02}", word, i));
    }

    variants
}

fn symbol_layer(word: &str, policy: &PasswordPolicy) -> Vec<String> {
    if !policy.require_symbol {
        return vec![word.to_string()];
    }

    SYMBOLS.iter()
        .flat_map(|s| vec![
            format!("{}{}", word, s),
            format!("{}{}", s, word),
        ])
        .collect()
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
    let categories = collect_dynamic_categories(persona);
    let years = derive_all_year_fragments(persona);
    let policy = &persona.policy;

    let base_patterns = cross_categories(&categories);

    base_patterns.par_iter()
        .flat_map(|pattern| {

            let mut expanded = Vec::new();

            expanded.push(pattern.clone());

            for year in &years {
                expanded.push(format!("{}{}", pattern, year));
                expanded.push(format!("{}{}", year, pattern));
            }

            expanded
        })
        .flat_map(|candidate| {

            // Allow shorter candidates to expand via mask layer
        if candidate.len() > policy.max_length {
            return Vec::new();
            }

            let enforced = enforce_case(candidate, policy);

            numeric_layer(&enforced, policy)
                .into_iter()
                .flat_map(|n| symbol_layer(&n, policy))
                .flat_map(|s| leet_layer(&s))
                .filter(|final_word| length_ok(final_word, policy))
                .collect::<Vec<String>>()
        })
        .filter(|candidate| validate(candidate, policy))
        .collect()
}