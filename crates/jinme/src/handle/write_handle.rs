use crate::prelude::*;
use std::{
    io,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct WriteHandle(Arc<Mutex<dyn io::Write + Send + Sync>>);

impl WriteHandle {
    pub fn new(writer: impl io::Write + Send + Sync + 'static) -> Self {
        Self(Arc::new(Mutex::new(writer)))
    }
    /// Get the inner Arc<Mutex> directly to avoid nested locks
    pub fn inner(&self) -> std::sync::Arc<Mutex<dyn io::Write + Send + Sync>> {
        self.0.clone()
    }
}

impl IHandle for WriteHandle {}
