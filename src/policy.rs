use crate::models::PasswordPolicy;

pub fn validate(word: &str, policy: &PasswordPolicy) -> bool {
    if word.len() < policy.min_length || word.len() > policy.max_length {
        return false;
    }

    if policy.require_upper && !word.chars().any(|c| c.is_uppercase()) {
        return false;
    }

    if policy.require_lower && !word.chars().any(|c| c.is_lowercase()) {
        return false;
    }

    if policy.require_numeric && !word.chars().any(|c| c.is_numeric()) {
        return false;
    }

    if policy.require_symbol && !word.chars().any(|c| !c.is_alphanumeric()) {
        return false;
    }

    for required in &policy.mandatory_include {
        if !word.contains(required) {
            return false;
        }
    }

    for excluded in &policy.exclude {
        if word.contains(excluded) {
            return false;
        }
    }

    true
}