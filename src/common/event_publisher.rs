use std::fmt::Formatter;
use crate::common::Event;

#[derive(Debug)]
pub enum EventPublishError {
    SerializationError,
    PersistentError
}

impl std::fmt::Display for EventPublishError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventPublishError::SerializationError => write!(f,"serialization error"),
            EventPublishError::PersistentError => write!(f,"persistent error"),
        }
    }
}

impl std::error::Error for EventPublishError {}

pub type RawEvent = (&'static str, String);

#[tonic::async_trait]
pub trait EventPublisherClient {
    async fn publish(&self, event: RawEvent) -> Result<(), EventPublishError>;
}

pub struct EventPublisher {
    client: Box<dyn EventPublisherClient + Send + Sync>,
}

impl EventPublisher {
    pub fn build(client: Box<dyn EventPublisherClient + Send + Sync>) -> EventPublisher {
        EventPublisher {
            client
        }
    }

    pub async fn publish<T: Event>(&self, event: &T) -> Result<(), EventPublishError> {
        let kind = event.kind();
        let data = serde_json::to_string(event)
            .map_err(|_| EventPublishError::SerializationError)?;

        self.client.publish((kind, data)).await
    }
}