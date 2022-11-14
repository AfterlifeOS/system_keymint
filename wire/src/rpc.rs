//! Local types that are equivalent to those generated for the IRemotelyProvisionedComponent HAL
//! interface

use crate::{cbor_type_error, try_from_n, AsCborValue, CborError};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use enumn::N;
use kmr_derive::AsCborValue;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ErrorCode {
    Ok = 0, // not in HAL, assumed
    Failed = 1,
    InvalidMac = 2,
    ProductionKeyInTestRequest = 3,
    TestKeyInProductionRequest = 4,
    InvalidEek = 5,
    Removed = 6,
}

/// The default value for the minimum number of keys supported in a CSR.
pub const MINIMUM_SUPPORTED_KEYS_IN_CSR: i32 = 20;

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct HardwareInfo {
    pub version_number: i32,
    pub rpc_author_name: String,
    pub supported_eek_curve: EekCurve,
    pub unique_id: Option<String>,
    pub supported_num_keys_in_csr: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsCborValue, N)]
#[repr(i32)]
pub enum EekCurve {
    None = 0,
    P256 = 1,
    Curve25519 = 2,
}
try_from_n!(EekCurve);

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct MacedPublicKey {
    pub maced_key: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct ProtectedData {
    pub protected_data: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, AsCborValue)]
pub struct DeviceInfo {
    pub device_info: Vec<u8>,
}
