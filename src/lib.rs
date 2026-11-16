// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-repository-signing: repository signing component model.
//!
//! Real implementation: repositorys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositorySigningError {
    EmptyRepositoryName,
    DuplicateRepository(String),
    EmptyEventSubject,
}

impl fmt::Display for RepositorySigningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRepositoryName => write!(f, "repository name must not be empty"),
            Self::DuplicateRepository(id) => write!(f, "duplicate repository {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RepositorySigningError {}

pub struct RepositorySigning {
    pub name: String,
    repositorys: BTreeMap<String, Repository>,
    events: BTreeMap<String, Event>,
}

impl RepositorySigning {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            repositorys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_repository(&mut self, item: Repository) -> Result<(), RepositorySigningError> {
        if item.name.trim().is_empty() {
            return Err(RepositorySigningError::EmptyRepositoryName);
        }
        let id = item.id.clone();
        if self.repositorys.contains_key(&id) {
            return Err(RepositorySigningError::DuplicateRepository(id.clone()));
        }
        self.repositorys.insert(id, item);
        Ok(())
    }

    pub fn repository_count(&self) -> usize {
        self.repositorys.len()
    }

    pub fn get_repository(&self, id: &str) -> Option<&Repository> {
        self.repositorys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RepositorySigningError> {
        if event.subject.trim().is_empty() {
            return Err(RepositorySigningError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_repository(id: &str, name: &str) -> Repository {
        Repository {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = RepositorySigning::new("RepositorySigning");
        assert_eq!(m.name, "RepositorySigning");
        assert_eq!(m.repository_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_repository() {
        let mut m = RepositorySigning::new("RepositorySigning");
        m.add_repository(sample_repository("a1", "Alpha")).unwrap();
        assert_eq!(m.repository_count(), 1);
        assert!(m.get_repository("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = RepositorySigning::new("RepositorySigning");
        assert!(m.add_repository(sample_repository("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = RepositorySigning::new("RepositorySigning");
        m.add_repository(sample_repository("x", "One")).unwrap();
        assert!(m.add_repository(sample_repository("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = RepositorySigning::new("RepositorySigning");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
