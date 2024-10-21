use std::sync::{Arc, Mutex};
use libsignal_protocol::Timestamp;
use uuid::{Uuid};

use common::signal_protobuf::{envelope, Envelope};
use libsignal_core::{DeviceId, ServiceId};

// This should be in one place (also in server.rs)
type UserID = u32;


/// Value Object pattern
/// Immutable
/// No identity
/// Structural equality
/// Defined by its values
pub struct RemovedMessage {
    src_service_id: Option<ServiceId>,
    dst_service_id: Option<ServiceId>,
    server_guid: Uuid,
    server_time_stamp: Timestamp,
    client_time_stamp: Timestamp,
    envelope_type: envelope::Type, 
}

impl RemovedMessage {
    pub fn from_envelope(envelope: Envelope) -> RemovedMessage {
        RemovedMessage {
            src_service_id: Some(ServiceId::parse_from_service_id_string(envelope.source_service_id())).unwrap_or_default(),
            dst_service_id: Some(ServiceId::parse_from_service_id_string(envelope.destination_service_id())).unwrap_or_default(),
            server_guid: Uuid::parse_str(envelope.server_guid()).unwrap_or_default(),
            server_time_stamp: Timestamp::from_epoch_millis(envelope.server_timestamp()),
            client_time_stamp: Timestamp::from_epoch_millis(envelope.client_timestamp()),
            envelope_type:envelope.r#type(),
        }
    }
}

pub struct DummyMessagesCache {
    pub cache: Arc<Mutex<(Uuid, DeviceId, Envelope)>>,
}

impl DummyMessagesCache {
    fn insert(
        &self,
        message_guid: Uuid,
        destination_device: DeviceId,
        message: Envelope,
    ) {
       let mut cache = self.cache.lock().unwrap(); 
       *cache = (message_guid, destination_device, message);
    }

     // public CompletableFuture<Optional<RemovedMessage>> remove(final UUID destinationUuid, final byte destinationDevice,
     //  final UUID messageGuid)
     //  public CompletableFuture<List<RemovedMessage>> remove(final UUID destinationUuid, final byte destinationDevice,
      // final List<UUID> messageGuids)
    // fn remove(&self, dst_uuid: Uuid, dst_device_id: DeviceId, message_guid: Uuid) -> Option<RemovedMessage> {
    //     // remove_by_guid.execute
    // }



    /// Generate message queue key for user
    fn get_message_queue_key(account_uuid: Uuid, device_id: DeviceId) -> Vec<u8> {
        return format!("user_queue::{{{}::{}}}", account_uuid.to_string(), device_id.to_string()).into_bytes();
    }

    /// Generate message queue meta key for user
    /// Statistics?
    fn get_message_queue_metadata_key(account_uuid: Uuid, device_id: DeviceId) -> Vec<u8> {
        return format!("user_queue_metadata::{{{}::{}}}", account_uuid.to_string(), device_id.to_string()).into_bytes();
    }

    /// Generate queue index for user
    /// Message retrieval?
    fn get_queue_index_key(account_uuid: Uuid, device_id: DeviceId) -> Vec<u8> {
        return format!("user_queue_index::{{{}::{}}}", account_uuid.to_string(), device_id.to_string()).into_bytes();
    }

    // pub fn get_queue_index_key(&self, slot: u128) -> Vec<u8> {
    //     return format!("user_queue_index::{{{}}}", slot).into_bytes();
    // }

    // Generate shared Message Router key
    // Generate key for tracking if user queue is currently persisting

}

struct MessagesManager {
    cache: DummyMessagesCache,
}

impl MessagesManager {
    /// Store message in cache and log
    /// public void insert(UUID destinationUuid, byte destinationDevice, Envelope message)
    /// messagesCache.insert(messageGuid, destinationUuid, destinationDevice, message);
    pub fn insert(&self, dst_uuid: Uuid, dst_device_id: DeviceId, message: Envelope) {
        self.cache.insert(dst_uuid, dst_device_id, message);
    }
    /// Delete message from cache or DB and log
    // pub fn delete(&self) {
    //     self.cache.delete(message_guid, destination_device, message)
    // }

    /// Add listener that notify device when message become available
    /// CompletableFuture<Optional<RemovedMessage>> delete(UUID destinationUuid, Device destinationDevice, UUID guid,
      // @Nullable Long serverTimestamp)
    // public CompletableFuture<List<RemovedMessage>> remove(final UUID destinationUuid, final byte destinationDevice,
      // final List<UUID> messageGuids)
    pub fn add_message_availability_listener(&self) {}

    /// Remove listener that notify device when message become available
    pub fn remove_message_availability_listener(&self) {}
}

#[cfg(test)]
mod messages_manager_tests {
    use std::sync::{Arc, Mutex};

    use common::signal_protobuf::Envelope;
    use libsignal_core::DeviceId;
    use uuid::Uuid;

    use crate::messages_manager::MessagesManager;

    use super::DummyMessagesCache;

    fn cache() -> DummyMessagesCache {
        DummyMessagesCache {
            cache: Arc::new(Mutex::new((
                Uuid::parse_str("myuuid").unwrap(),
                DeviceId::from(1),
                Envelope {
                    r#type: Some(8),
                    source_service_id: Some(String::from("source_service_id")),
                    source_device: Some(u32::from(DeviceId::from(2))),
                    client_timestamp: Some(1),
                    // Hej Bob!
                    content: Some(vec![0x48, 0x65, 0x61, 0x20, 0x42, 0x6f, 0x62, 0x21]),
                    server_guid: Some(String::from("server_guid")),
                    server_timestamp: Some(1),
                    ephemeral: Some(false),
                    destination_service_id: Some(String::from("destination_service_id")),
                    urgent: Some(false),
                    updated_pni: Some(String::from("pni")),
                    story: Some(false),
                    report_spam_token: None,
                    shared_mrm_key: None,
                },
            ))),
        }
    }

    #[test]
    fn my_test() {
        let cache = cache();
        let messages_manager = MessagesManager { cache };

        let uuid = Uuid::parse_str("myuuid").unwrap();
        let device_id: DeviceId = 3.into();
        let envelope = Envelope {
            r#type: Some(8),
            source_service_id: Some(String::from("source_service_id")),
            source_device: Some(u32::from(DeviceId::from(2))),
            client_timestamp: Some(1),
            // Hej Bob!
            content: Some(vec![0x48, 0x65, 0x61, 0x20, 0x42, 0x6f, 0x62, 0x21]),
            server_guid: Some(String::from("server_guid")),
            server_timestamp: Some(1),
            ephemeral: Some(false),
            destination_service_id: Some(String::from("destination_service_id")),
            urgent: Some(false),
            updated_pni: Some(String::from("pni")),
            story: Some(false),
            report_spam_token: None,
            shared_mrm_key: None,
        };
        messages_manager.insert(uuid, device_id, envelope);
        let cache_content = cache.cache.lock().unwrap();
        assert_eq!(1, 1)
    }
}
