use crate::prelude::*;
use std::{
    io,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct BufReadHandle(Arc<Mutex<dyn io::BufRead + Send + Sync>>);

impl BufReadHandle {
    pub fn new(writer: impl io::BufRead + Send + Sync + 'static) -> Self {
        Self(Arc::new(Mutex::new(writer)))
    }
    /// Get the inner Arc<Mutex> directly to avoid nested locks
    pub fn inner(&self) -> std::sync::Arc<Mutex<dyn io::BufRead + Send + Sync>> {
        self.0.clone()
    }
}

impl IHandle for BufReadHandle {}
