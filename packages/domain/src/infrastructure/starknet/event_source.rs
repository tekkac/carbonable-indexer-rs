use std::collections::HashMap;

use apibara_core::starknet::v1alpha2::{BlockHeader, EventWithTransaction};
use time::OffsetDateTime;

use crate::domain::event_source::{BlockMetadata, DomainEvent, Filterable};

impl DomainEvent {
    pub fn from_starknet_event(
        value: EventWithTransaction,
        application_filter: &mut [Box<dyn Filterable>],
        last_event_idx: &mut usize,
        last_processed_felt: &mut String,
    ) -> Self {
        let meta = &value
            .transaction
            .clone()
            .expect("transaction is required")
            .meta
            .expect("meta is required");

        let felt = &meta.hash.clone().expect("hash is required");
        // Doing some magic to keep track of the previously handled events
        if felt.to_hex() != *last_processed_felt {
            *last_processed_felt = felt.to_hex();
            *last_event_idx = 0;
        }
        let event = &value.event.clone().expect("event should not be empty");

        let mut version = &value
            .receipt
            .clone()
            .unwrap()
            .events
            .iter()
            .skip(*last_event_idx)
            .position(|e| *e == event.clone())
            .unwrap_or(0);
        // Some magic to keep track of the "real" version
        let next_version = &(*version + *last_event_idx);
        version = next_version;
        *last_event_idx = *version;

        let mut payload: HashMap<String, String> = HashMap::new();

        event.data.iter().enumerate().for_each(|(i, data)| {
            payload.insert(i.to_string(), data.to_string());
        });

        let from = event
            .from_address
            .clone()
            .expect("from address is required");
        let key = event.keys.first().expect("key should exist");
        if 0 == event.data.len() {
            event.keys.iter().skip(1).enumerate().for_each(|(i, data)| {
                payload.insert(i.to_string(), data.to_string());
            });
        }

        // Will panic if event is none. Event should be extracted out of application filters
        let mut event = None;
        for filter in application_filter {
            let found_event = filter.get_event(&from.to_string(), &key.to_string());
            if found_event.is_some() {
                event = found_event;
                continue;
            }
        }
        if event.is_none() {
            tracing::error!("FROM {} KEY {}", from, key);
        }

        let mut metadata = HashMap::new();
        add_tx_hash(&mut metadata, &value);
        add_contract_event_emitter(&mut metadata, &value);

        DomainEvent {
            id: format!("{felt}_{version}"),
            metadata,
            payload,
            r#type: event.expect("event should not be none at this point"),
        }
    }
}

/// Search for transaction hash from [`EventWithTransaction`]
/// and adds it to metadata [`HashMap`]
fn add_tx_hash(metadata: &mut HashMap<String, String>, value: &EventWithTransaction) {
    if let Some(receipt) = &value.receipt {
        if let Some(tx_hash) = &receipt.transaction_hash {
            metadata.insert("tx_hash".to_string(), tx_hash.to_string());
        }
    }
}

/// Search for contract event emitter address from [`EventWithTransaction`]
/// and adds it to metadata [`HashMap`]
fn add_contract_event_emitter(
    metadata: &mut HashMap<String, String>,
    value: &EventWithTransaction,
) {
    if let Some(event) = &value.event {
        if let Some(from_address) = &event.from_address {
            metadata.insert("from_address".to_string(), from_address.to_string());
        }
    }
}

impl From<BlockHeader> for BlockMetadata {
    fn from(value: BlockHeader) -> Self {
        let recorded_at = OffsetDateTime::from_unix_timestamp(
            value.timestamp.expect("should have timestamp").seconds,
        )
        .expect("recorded_at should be convertible to datetime");
        Self {
            hash: value.block_hash.expect("should have hash").to_string(),
            number: value.block_number,
            timestamp: recorded_at,
        }
    }
}
