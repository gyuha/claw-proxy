use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use crate::providers::Provider;
use crate::config::RoutingStrategy;
use crate::error::AppError;

pub mod round_robin;
pub mod failover;

pub struct Router {
    providers: Vec<Arc<Box<dyn Provider>>>,
    strategy: RoutingStrategy,
    counter: AtomicUsize,
}

impl Router {
    pub fn new(providers: Vec<Box<dyn Provider>>, strategy: RoutingStrategy) -> Self {
        Self {
            providers: providers.into_iter().map(Arc::new).collect(),
            strategy,
            counter: AtomicUsize::new(0),
        }
    }

    pub fn next_provider(&self) -> Result<Arc<Box<dyn Provider>>, AppError> {
        match self.strategy {
            RoutingStrategy::RoundRobin => round_robin::next(&self.providers, &self.counter),
            RoutingStrategy::Failover => failover::next(&self.providers),
        }
    }

    pub fn providers(&self) -> &[Arc<Box<dyn Provider>>] {
        &self.providers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::normalizer::{InternalRequest, InternalResponse, ApiFormat};
    use async_trait::async_trait;

    struct MockProvider { name: String, available: bool }

    #[async_trait]
    impl Provider for MockProvider {
        fn name(&self) -> &str { &self.name }
        fn is_available(&self) -> bool { self.available }
        fn supports_model(&self, _: &str) -> bool { true }
        async fn chat_completion(&self, _: InternalRequest) -> Result<InternalResponse, AppError> {
            Ok(InternalResponse {
                id: "test".to_string(),
                model: "test".to_string(),
                content: format!("response from {}", self.name),
                input_tokens: 10,
                output_tokens: 5,
            })
        }
    }

    #[tokio::test]
    async fn test_round_robin_cycles() {
        let providers: Vec<Box<dyn Provider>> = vec![
            Box::new(MockProvider { name: "p1".to_string(), available: true }),
            Box::new(MockProvider { name: "p2".to_string(), available: true }),
        ];
        let router = Router::new(providers, RoutingStrategy::RoundRobin);
        assert_eq!(router.next_provider().unwrap().name(), "p1");
        assert_eq!(router.next_provider().unwrap().name(), "p2");
        assert_eq!(router.next_provider().unwrap().name(), "p1");
    }

    #[tokio::test]
    async fn test_failover_picks_first_available() {
        let providers: Vec<Box<dyn Provider>> = vec![
            Box::new(MockProvider { name: "p1".to_string(), available: true }),
            Box::new(MockProvider { name: "p2".to_string(), available: false }),
        ];
        let router = Router::new(providers, RoutingStrategy::Failover);
        assert_eq!(router.next_provider().unwrap().name(), "p1");
    }

    #[tokio::test]
    async fn test_failover_skips_unavailable() {
        let providers: Vec<Box<dyn Provider>> = vec![
            Box::new(MockProvider { name: "p1".to_string(), available: false }),
            Box::new(MockProvider { name: "p2".to_string(), available: true }),
        ];
        let router = Router::new(providers, RoutingStrategy::Failover);
        assert_eq!(router.next_provider().unwrap().name(), "p2");
    }
}