//! pzprjs puzzle ids and aliases (`pzpr.variety.toPID`).

use crate::variety_table::VARIETIES;

/// Resolve a pid or alias to the pzprjs pid, as `pzpr.variety.toPID` does.
pub fn to_pid(name: &str) -> Option<&'static str> {
    if let Some((pid, _, _)) = VARIETIES.iter().find(|(pid, _, _)| *pid == name) {
        return Some(pid);
    }
    VARIETIES
        .iter()
        .find(|(_, _, aliases)| aliases.contains(&name))
        .map(|(pid, _, _)| *pid)
}

/// The id pzprjs writes into URLs for a pid.
pub fn urlid(pid: &str) -> Option<&'static str> {
    VARIETIES
        .iter()
        .find(|(p, _, _)| *p == pid)
        .map(|(_, urlid, _)| *urlid)
}

/// All pzprjs pids, in declaration order.
pub fn all_pids() -> impl Iterator<Item = &'static str> {
    VARIETIES.iter().map(|(pid, _, _)| *pid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aliases() {
        assert_eq!(to_pid("masyu"), Some("mashu"));
        assert_eq!(to_pid("pearl"), Some("mashu"));
        assert_eq!(to_pid("slitherlink"), Some("slither"));
        assert_eq!(to_pid("slither"), Some("slither"));
        assert_eq!(to_pid("hashi"), Some("hashikake"));
        assert_eq!(urlid("hashikake"), Some("hashi"));
        assert_eq!(to_pid("nope"), None);
    }
}
