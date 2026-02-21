use std::io::{self, Write};
use crate::models::*;

fn prompt_vec(label: &str) -> Vec<String> {
    print!("{} (comma separated): ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn prompt_string(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_bool(label: &str) -> bool {
    print!("{} (y/n): ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

pub fn run_interview() -> Persona {
    println!("--- OmniProfile-Gen Interactive Persona Interview ---");

    let identity = Identity {
        full_name: prompt_string("Full name"),
        nicknames: prompt_vec("Nicknames"),
        spouse: prompt_vec("Spouse/Partner names"),
        children: prompt_vec("Children names"),
        pets: prompt_vec("Pets"),
        maiden_names: prompt_vec("Maiden names"),
    };

    let chronology = Chronology {
        birthdates: prompt_vec("Birthdates (DDMMYYYY or YYYY)"),
        anniversaries: prompt_vec("Anniversaries"),
        graduation_years: prompt_vec("Graduation years"),
        employment_start: prompt_vec("Employment start dates"),
    };

    let geography = Geography {
        birth_city: prompt_vec("Birth city"),
        current_city: prompt_vec("Current city"),
        streets: prompt_vec("Street names"),
        vacation_spots: prompt_vec("Vacation spots"),
    };

    let professional = Professional {
        current_company: prompt_vec("Current company"),
        previous_employers: prompt_vec("Previous employers"),
        departments: prompt_vec("Departments"),
        projects: prompt_vec("Project codenames"),
        host_naming: prompt_vec("Internal host naming patterns"),
    };

    let personal = Personal {
        sports_teams: prompt_vec("Sports teams"),
        bands: prompt_vec("Favorite bands"),
        hobbies: prompt_vec("Hobbies"),
        cars: prompt_vec("Car make/model"),
    };

    let culture = Culture {
        slang: prompt_vec("Common slang"),
        keyboard_layout: prompt_string("Keyboard layout (qwerty/azerty/etc)"),
    };

    println!("--- Password Policy ---");

    let policy = PasswordPolicy {
        min_length: prompt_string("Minimum length").parse().unwrap_or(8),
        max_length: prompt_string("Maximum length").parse().unwrap_or(64),
        require_upper: prompt_bool("Require uppercase"),
        require_lower: prompt_bool("Require lowercase"),
        require_numeric: prompt_bool("Require numeric"),
        require_symbol: prompt_bool("Require symbol"),
        mandatory_include: prompt_vec("Mandatory inclusion strings"),
        exclude: prompt_vec("Exclude strings"),
    };

    let output_file = prompt_string("Output filename");

    Persona {
        identity,
        chronology,
        geography,
        professional,
        personal,
        culture,
        policy,
        output_file,
    }
}