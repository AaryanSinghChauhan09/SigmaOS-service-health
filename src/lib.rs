// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-service-health: service health component model.
//!
//! Real implementation: healths, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Health {
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
pub enum ServiceHealthError {
    EmptyHealthName,
    DuplicateHealth(String),
    EmptyEventSubject,
}

impl fmt::Display for ServiceHealthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHealthName => write!(f, "health name must not be empty"),
            Self::DuplicateHealth(id) => write!(f, "duplicate health {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ServiceHealthError {}

pub struct ServiceHealth {
    pub name: String,
    healths: BTreeMap<String, Health>,
    events: BTreeMap<String, Event>,
}

impl ServiceHealth {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            healths: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_health(&mut self, item: Health) -> Result<(), ServiceHealthError> {
        if item.name.trim().is_empty() {
            return Err(ServiceHealthError::EmptyHealthName);
        }
        let id = item.id.clone();
        if self.healths.contains_key(&id) {
            return Err(ServiceHealthError::DuplicateHealth(id.clone()));
        }
        self.healths.insert(id, item);
        Ok(())
    }

    pub fn health_count(&self) -> usize {
        self.healths.len()
    }

    pub fn get_health(&self, id: &str) -> Option<&Health> {
        self.healths.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ServiceHealthError> {
        if event.subject.trim().is_empty() {
            return Err(ServiceHealthError::EmptyEventSubject);
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

    fn sample_health(id: &str, name: &str) -> Health {
        Health {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ServiceHealth::new("ServiceHealth");
        assert_eq!(m.name, "ServiceHealth");
        assert_eq!(m.health_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_health() {
        let mut m = ServiceHealth::new("ServiceHealth");
        m.add_health(sample_health("a1", "Alpha")).unwrap();
        assert_eq!(m.health_count(), 1);
        assert!(m.get_health("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ServiceHealth::new("ServiceHealth");
        assert!(m.add_health(sample_health("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ServiceHealth::new("ServiceHealth");
        m.add_health(sample_health("x", "One")).unwrap();
        assert!(m.add_health(sample_health("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ServiceHealth::new("ServiceHealth");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
