use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::providers::Provider;
use crate::error::AppError;

pub fn next(
    providers: &[Arc<Box<dyn Provider>>],
    counter: &AtomicUsize,
) -> Result<Arc<Box<dyn Provider>>, AppError> {
    let available: Vec<_> = providers.iter().filter(|p| p.is_available()).collect();
    if available.is_empty() {
        return Err(AppError::NoProviders);
    }
    let idx = counter.fetch_add(1, Ordering::Relaxed) % available.len();
    Ok(Arc::clone(available[idx]))
}