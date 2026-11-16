//! SigmaOS-amf: AMD Advanced Media Framework model.
//!
//! Real implementation: encoders, streams, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encoder {
    pub id: String,
    pub name: String,
    pub codec: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream {
    pub id: String,
    pub encoder_id: String,
    pub bitrate: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmfError {
    EmptyEncoderName,
    DuplicateEncoder(String),
}

impl fmt::Display for AmfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEncoderName => write!(f, "encoder name must not be empty"),
            Self::DuplicateEncoder(id) => write!(f, "duplicate encoder {:?}", id),
        }
    }
}

impl std::error::Error for AmfError {}

pub struct Amf {
    pub name: String,
    encoders: BTreeMap<String, Encoder>,
    streams: BTreeMap<String, Stream>,
}

impl Amf {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), encoders: BTreeMap::new(), streams: BTreeMap::new() }
    }

    pub fn add_encoder(&mut self, e: Encoder) -> Result<(), AmfError> {
        if e.name.trim().is_empty() { return Err(AmfError::EmptyEncoderName); }
        let id = e.id.clone();
        if self.encoders.contains_key(&id) {
            return Err(AmfError::DuplicateEncoder(id.clone()));
        }
        self.encoders.insert(id, e);
        Ok(())
    }

    pub fn encoder_count(&self) -> usize { self.encoders.len() }

    pub fn get_encoder(&self, id: &str) -> Option<&Encoder> { self.encoders.get(id) }

    pub fn add_stream(&mut self, s: Stream) {
        self.streams.insert(s.id.clone(), s);
    }

    pub fn get_stream(&self, id: &str) -> Option<&Stream> { self.streams.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Amf::new("Amf");
        assert_eq!(a.name, "Amf");
        assert_eq!(a.encoder_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Amf::new("Amf");
        let encoder = Encoder { id: "e1".into(), name: "h264".into(), codec: "AVC".into() };
        a.add_encoder(encoder).unwrap();
        assert_eq!(a.encoder_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Amf::new("Amf");
        let encoder = Encoder { id: "x".into(), name: "".into(), codec: "x".into() };
        assert!(a.add_encoder(encoder).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Amf::new("Amf");
        let encoder = Encoder { id: "x".into(), name: "X".into(), codec: "x".into() };
        a.add_encoder(encoder).unwrap();
        let encoder2 = Encoder { id: "x".into(), name: "X".into(), codec: "x".into() };
        assert!(a.add_encoder(encoder2).is_err());
    }
}
