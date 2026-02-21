use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Identity {
    pub full_name: String,
    pub nicknames: Vec<String>,
    pub spouse: Vec<String>,
    pub children: Vec<String>,
    pub pets: Vec<String>,
    pub maiden_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chronology {
    pub birthdates: Vec<String>,
    pub anniversaries: Vec<String>,
    pub graduation_years: Vec<String>,
    pub employment_start: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Geography {
    pub birth_city: Vec<String>,
    pub current_city: Vec<String>,
    pub streets: Vec<String>,
    pub vacation_spots: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Professional {
    pub current_company: Vec<String>,
    pub previous_employers: Vec<String>,
    pub departments: Vec<String>,
    pub projects: Vec<String>,
    pub host_naming: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Personal {
    pub sports_teams: Vec<String>,
    pub bands: Vec<String>,
    pub hobbies: Vec<String>,
    pub cars: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Culture {
    pub slang: Vec<String>,
    pub keyboard_layout: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordPolicy {
    pub min_length: usize,
    pub max_length: usize,
    pub require_upper: bool,
    pub require_lower: bool,
    pub require_numeric: bool,
    pub require_symbol: bool,
    pub mandatory_include: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Persona {
    pub identity: Identity,
    pub chronology: Chronology,
    pub geography: Geography,
    pub professional: Professional,
    pub personal: Personal,
    pub culture: Culture,
    pub policy: PasswordPolicy,
    pub output_file: String,
}