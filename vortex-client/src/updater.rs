// -- leaked by @azixi0 on github
use std::cmp::Ordering;

pub const DISABLE_UPDATE_ENV: &str = "VORTEX_NO_UPDATE";
pub const TEMP_PREFIX: &str = "vortex_update_";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionInfo { pub version: String }

impl VersionInfo {
    pub fn is_newer_than(&self, current: &str) -> bool { compare_versions(&self.version, current) == Ordering::Greater }
}

fn compare_versions(left: &str, right: &str) -> Ordering {
    let parse = |value: &str| value.trim_start_matches('v').split('.')
        .map(|part| part.parse::<u64>().unwrap_or(0)).collect::<Vec<_>>();
    let (left, right) = (parse(left), parse(right));
    for index in 0..left.len().max(right.len()) {
        let order = left.get(index).copied().unwrap_or(0).cmp(&right.get(index).copied().unwrap_or(0));
        if order != Ordering::Equal { return order; }
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn compares_versions() { assert!(VersionInfo { version: "0.4.3".into() }.is_newer_than("0.4.2")); }
}

