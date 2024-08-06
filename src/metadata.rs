use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "__metadata__")]
    pub metadata: Metadata,

    #[serde(flatten)]
    pub weights: HashMap<String, Weight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weight {
    pub dtype: Dtype,

    pub shape: Vec<i64>,

    pub data_offsets: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Dtype {
    #[serde(rename = "F64")]
    Fp64,
    #[serde(rename = "F32")]
    Fp32,
    #[serde(rename = "F16")]
    Fp16,
    #[serde(rename = "BF16")]
    Bf16,
    #[serde(rename = "I64")]
    Int64,
    #[serde(rename = "I32")]
    Int32,
    #[serde(rename = "I16")]
    Int16,
    #[serde(rename = "U8")]
    Uint8,
    #[serde(rename = "BOOL")]
    Bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub format: Option<TensorFormart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TensorFormart {
    #[serde(rename = "pt")]
    PyTorch,
    #[serde(rename = "tf")]
    TensorFlow,
}
