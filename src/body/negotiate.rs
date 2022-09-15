use std::io::Bytes;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::body::{Capabilities, FileTime, SecurityMode};
use crate::byte_helper::{bytes_to_u16, bytes_to_u32, u16_to_bytes, u32_to_bytes};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SMBNegotiationRequestBody {
    security_mode: SecurityMode,
    capabilities: Capabilities,
    client_uuid: Uuid,
    dialects: Vec<SMBDialect>,
    // negotiate_contexts: Vec
}

impl SMBNegotiationRequestBody {
    pub fn from_bytes(bytes: &[u8]) -> Option<(Self, &[u8])> {
        if bytes.len() < 37 { return None }
        let dialect_count = bytes_to_u16(&bytes[2..4]) as usize;
        let security_mode = SecurityMode::from_bits_truncate(bytes[4]);
        let capabilities = Capabilities::from_bits_truncate(bytes[8]);
        let client_uuid = Uuid::from_slice(&bytes[12..28]).ok()?;
        let mut dialects = Vec::new();
        let mut dialect_idx = 36;
        while dialects.len() < dialect_count {
            let dialect_code = bytes_to_u16(&bytes[dialect_idx..(dialect_idx+2)]);
            if let Ok(dialect) = SMBDialect::try_from(dialect_code) {
                dialects.push(dialect);
            }
            dialect_idx += 2;
        }
        if dialects.contains(&SMBDialect::V3_1_1) {
            let negotiate_ctx_idx = bytes_to_u32(&bytes[28..32]);
            let negotiate_ctx_cnt = bytes_to_u16(&bytes[32..34]);
            // TODO add negotiate ctx parsing
        }
        Some((Self { security_mode, capabilities, client_uuid, dialects }, &bytes[dialect_idx..]))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SMBNegotiationResponseBody {
    security_mode: SecurityMode,
    dialect: SMBDialect,
    guid: Uuid,
    capabilities: Capabilities,
    max_transact_size: u32,
    max_read_size: u32,
    max_write_size: u32,
    system_time: FileTime,
    server_start_time: FileTime,
    buffer: Vec<u8>
}

impl SMBNegotiationResponseBody {
    pub fn new(security_mode: SecurityMode, dialect: SMBDialect, capabilities: Capabilities, max_transact_size: u32, max_read_size: u32, max_write_size: u32, server_start_time: FileTime, buffer: Vec<u8>) -> Self {
        Self {
            security_mode,
            dialect,
            guid: Uuid::new_v4(),
            capabilities,
            max_transact_size,
            max_read_size,
            max_write_size,
            system_time: FileTime::now(),
            server_start_time,
            buffer
        }
    }
}

impl SMBNegotiationResponseBody {
    pub fn as_bytes(&self) -> Vec<u8> {
        [
            &[65, 0][0..], // Structure Size
            &[self.security_mode.bits(), 0],
            &u16_to_bytes(self.dialect as u16),
            &*self.guid.as_bytes(),
            &u32_to_bytes(self.capabilities.bits() as u32),
            &u32_to_bytes(self.max_transact_size),
            &u32_to_bytes(self.max_read_size),
            &u32_to_bytes(self.max_write_size),
            &*self.system_time.as_bytes(),
            &*self.server_start_time.as_bytes(),
            &[64, 0], // Security Buffer Offset
            &u16_to_bytes(self.buffer.len() as u16),
            &[0; 4], // NegotiateContextOffset/Reserved/TODO
            &*self.buffer
            // TODO padding & NegotiateContextList
        ].concat()
    }
}

#[repr(u16)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Serialize, Deserialize, Copy, Clone)]
pub enum SMBDialect {
    V2_0_2 = 0x202,
    V2_1_0 = 0x210,
    V3_0_0 = 0x300,
    V3_0_2 = 0x302,
    V3_1_1 = 0x311,
    V2_X_X = 0x2FF
}

#[repr(u16)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Serialize, Deserialize)]
pub enum NegotiateContext {
    PreAuthIntegrityCapabilities = 0x01,
    EncryptionCapabilities,
    CompressionCapabilities,
    NetnameNegotiateContextID = 0x05,
    TransportCapabilities,
    RDMATransformCapabilities,
    SigningCapabilities
}