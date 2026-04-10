use std::sync::Arc;
use crate::providers::Provider;
use crate::error::AppError;

pub fn next(providers: &[Arc<Box<dyn Provider>>]) -> Result<Arc<Box<dyn Provider>>, AppError> {
    providers.iter()
        .find(|p| p.is_available())
        .map(Arc::clone)
        .ok_or(AppError::NoProviders)
}