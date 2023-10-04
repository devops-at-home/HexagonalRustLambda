use serde::{Deserialize, Serialize};

use crate::events::event_emmiter::SerialisableEvent;

const EVENT_TYPE: &str = "product_updated";

#[derive(Serialize, Deserialize, Clone)]
pub struct EventProductUpdatedV1 {
    pub version: u32,
    pub event_type: String,
    pub product: models::models::product::Product,
}

impl EventProductUpdatedV1 {
    pub fn new(product: models::models::product::Product) -> Self {
        Self {
            version: 1,
            event_type: EVENT_TYPE.to_string(),
            product,
        }
    }
}

impl SerialisableEvent for EventProductUpdatedV1 {
    fn get_event_type(&self) -> &String {
        &self.event_type
    }

    fn get_version(&self) -> u32 {
        self.version
    }

    fn serialise(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
