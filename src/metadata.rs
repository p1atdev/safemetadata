use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;

pub type Weights = BTreeMap<String, Weight>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// Metadata information about the model.
    #[serde(rename = "__metadata__")]
    pub metadata: Metadata,

    /// The model's weights, stored as a map from tensor names to weights.
    #[serde(flatten)]
    pub weights: Weights,
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

impl Display for Dtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dtype::Fp64 => write!(f, "float64"),
            Dtype::Fp32 => write!(f, "float32"),
            Dtype::Fp16 => write!(f, "float16"),
            Dtype::Bf16 => write!(f, "bfloat16"),
            Dtype::Int64 => write!(f, "int64"),
            Dtype::Int32 => write!(f, "int32"),
            Dtype::Int16 => write!(f, "int16"),
            Dtype::Uint8 => write!(f, "uint8"),
            Dtype::Bool => write!(f, "bool"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TensorFormart {
    #[serde(rename = "pt")]
    PyTorch,
    #[serde(rename = "tf")]
    TensorFlow,
    #[serde(rename = "np")]
    NumPy,
    #[serde(rename = "paddle")]
    Paddle,
    #[serde(rename = "flax")]
    Flax,
}

impl Display for TensorFormart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TensorFormart::PyTorch => write!(f, "PyTorch"),
            TensorFormart::TensorFlow => write!(f, "TensorFlow"),
            TensorFormart::NumPy => write!(f, "NumPy"),
            TensorFormart::Paddle => write!(f, "Paddle"),
            TensorFormart::Flax => write!(f, "Flax"),
        }
    }
}

/// Stability AI Model Metadata Standard.
/// See https://github.com/Stability-AI/ModelSpec?tab=readme-ov-file#specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSpec {
    /// Mandatory identifier key, indicates the presence and version of this specification.
    /// Trainer tools that support the spec should automatically emit this key,
    /// set to the version they support.
    #[serde(rename = "modelspec.sai_model_spec")]
    pub sai_model_spec: Option<String>,

    /// The specific classifier of the model's architecture,
    /// must be unique between models that have different inferencing
    /// requirements (*so for example SDV2-512 and SDv2-768-v are different
    /// enough that the distinction must be marked here, as the code must
    /// behave different to support it*). Simple finetunes of a model do not
    /// require a unique class, as the inference code does not have to
    /// change to support it. See architecture ID listing below this table
    /// for specific examples.
    /// The `/` slash symbol is defined as an optional meaningful separator.
    /// For example, `stable-diffusion-v1/lora` indicates that it is
    /// a LoRA trained to be applied to a `stable-diffusion-v1` model.
    /// Implementations are not required to parse this separator
    /// unless it is useful to them to process it.
    #[serde(rename = "modelspec.architecture")]
    pub architecture: Option<String>,

    /// A reliably static string that identifies the standard
    /// implementation codebase. The model's tensor keys generally
    /// will match variable names within the official codebase.
    /// This can be a named like `sgm` or a GitHub URL like
    /// `https://github.com/Stability-AI/generative-models` or
    /// any other string, as long as you do not change it between
    /// different models of the same format.
    #[serde(rename = "modelspec.implementation")]
    pub implementation: Option<String>,

    /// A title unique to the specific model. Generally for end-user training
    /// software, the user should provide this. If they do not, one can be
    /// provided as just eg the original file name or training run name.
    /// Inference UIs are encouraged to display this title to users in any
    /// model selector tools.
    #[serde(rename = "modelspec.title")]
    pub title: Option<String>,

    /// A user-friendly textual description of the model. This may describe what
    /// the model is trained on, what its capabilities are, or specific data
    /// like trigger words for a small SD finetunes. This field is permitted
    /// to contain very long sections of text, with paragraphs and etc. Inference
    /// UIs are encouraged to make this description visible-but-not-in-the-way to
    /// end users. Usage of markdown formatting is encouraged, and UIs are
    /// encouraged to format the markdown properly (displaying as plaintext is
    /// also acceptable where markdown is not possible).
    #[serde(rename = "modelspec.description")]
    pub description: Option<String>,

    /// The name or identity of the company or individual that created a model.
    /// Can even be a username or personal profile link.
    #[serde(rename = "modelspec.author")]
    pub author: Option<String>,

    /// The precise date that a model was created or published, in any ISO-8601-compliant format.
    #[serde(rename = "modelspec.date")]
    pub date: Option<String>,

    /// A hash of all tensor content (ie excluding the header section), with 0x prefix,
    /// all lowercase, and no byte-separator symbols. Other keys with the hash_ prefix
    /// followed by a different hash algorithm (eg hash_md5) are expected to obey the same
    /// format rules and implement the hash algorithm named within. Future versions of
    /// the spec may change which algorithm is encouraged as `SHOULD`. Inferencing engines are
    /// encouraged to validate that the hash matches after loading a file and warn the user
    /// if it does not match (ie possible file corruption). Model trainers/modifiers are
    /// strongly encouraged to calculate the hash and emit it correctly automatically
    /// whenever saving a model. This is not a MUST because hash algorithms may change
    /// with time, and the format should not be locked in to just one.
    #[serde(rename = "modelspec.hash_sha256")]
    pub hash_sha256: Option<String>,

    /// A minimum required version of the specified `implementation` codebase.
    /// This can be an actual version ID (eg `2.0.0`) or a commit hash.
    #[serde(rename = "modelspec.implementation_version")]
    pub implementation_version: Option<String>,

    /// If the model is under any form of license terms or restrictions, they should be
    /// clearly identified here. The model creator may at their own discretion (A) provide
    /// the name of the license, (B) provide a link to the license terms, or (C) emit
    /// the license terms in full in this slot.
    #[serde(rename = "modelspec.license")]
    pub license: Option<String>,

    /// Usage hint(s) for the model, where applicable. This field should be short,
    /// and just quickly describe bits of information a user might need while operating
    /// the model. Inference UIs are encouraged to make this information readily visible
    /// to the user when it is present. For example, a small SD finetune model would use
    /// this to list trigger words.
    #[serde(rename = "modelspec.usage_hint")]
    pub usage_hint: Option<String>,

    /// A (very small!) thumbnail icon in data-image format to be provided as a preview
    /// in inference UIs. Note that safetensors headers usually occupy a few hundreds of
    /// kilobytes, and don't get officially limited until 100 megabytes, so a small jpeg in
    /// data-image format does not significantly increase the size. 256x256 is a recommended
    /// size and aspect ratio (square).
    #[serde(rename = "modelspec.thumbnail")]
    pub thumbnail: Option<String>,

    /// An optional user-specified comma-separated list of category/tag labels for a finetuned
    /// model. A model trained on a specific person would be named after that person,
    /// but might be tagged as simply `Person`. Model listings can use this key when present
    /// to organize and allow easy filtering by tag. When in doubt on what tag(s) to use,
    /// the model maker is encouraged to look at the category label on other similar models,
    /// or choose a new label if they're the first to make a model in a category.
    #[serde(rename = "modelspec.tags")]
    pub tags: Option<String>,

    /// If the model was created by merging other models, you may provide a comma-separated
    /// list of the source models here. More details about merging or creation process
    /// may be included in the `description` key or in nonstandard keys. Models that do not
    /// provide this key are presumed to have been uniquely trained rather than merged.
    #[serde(rename = "modelspec.merged_from")]
    pub merged_from: Option<String>,

    // Image Generation Models
    /// The base resolution an image generator is intended to work at,
    /// in `(width)x(height)`` format. This does not need to account
    /// for aspect ratios. Future image generator models of a class
    /// that are able to handle any resolution may omit this key.
    /// Current generation Stable Diffusion models should always have
    /// this key. Note that adapter and component models can leave this off.
    ///
    /// Example: `512x512`, `1024x1024`
    #[serde(rename = "modelspec.resolution")]
    pub resolution: Option<String>,

    /// For image generation adapter models (eg LoRA) especially,
    /// if a model is trained to heavily require a phrase,
    /// it should be placed here. Inference UIs are welcomed to
    /// auto-emit this phrase into the prompt if it is present
    /// (encouraged to make this behavior optional to the user where possible).
    #[serde(rename = "modelspec.trigger_phrase")]
    pub trigger_phrase: Option<String>,

    /// In Stable Diffusion, `v` or `epsilon`. Other model classes
    /// may have their own concepts that apply.
    #[serde(rename = "modelspec.prediction_type")]
    pub prediction_type: Option<String>,

    /// If a model is tuned on a sub-section of possible timesteps
    /// (Timestep-Expert Models), identify it here, in the format `<min>,<max>``.
    #[serde(rename = "modelspec.timestep_range")]
    pub timestep_range: Option<String>,

    /// (Specialty) for "clip skip" in Stable Diffusion models,
    /// or similar practice in other models like it, this can be
    /// applied where relevant to identify that a non-standard
    /// layer of an encoder model should be used (so for example
    /// value `2` in an SD model indicates `clip_skip=2` should be used).
    #[serde(rename = "modelspec.encoder_layer")]
    pub encoder_layer: Option<String>,

    /// (Specialty) for "ControlNet" or similar model-adapter types
    /// that require preprocessing, this is an indicator of the
    /// preprocessing type, as a simple text identifier. Should not
    /// identify exact tool (eg "MiDaS"), just the broad type (eg "depth").
    #[serde(rename = "modelspec.preprocessor")]
    pub preprocessor: Option<String>,

    /// (Specialty) for "Textual-Inversion" or similar input-embedding
    /// types that modify prompts, this can be true to indicate that
    /// the embedding is meant for Negative Prompts, or false to
    /// indicate it's meant for (Positive) Prompts. A UI implementation
    /// may use this key to apply embeddings correctly with less user-intervention.
    #[serde(rename = "modelspec.is_negative_embedding")]
    pub is_negative_embedding: Option<bool>,

    /// (Specialty) for UNet based models that have special DType requirements
    /// (eg incompatible with fp16 but works with bf16) for inference,
    /// a comma-separated list of known-good types. Inference engines are
    /// recommended to ensure a compatible type is used when this is specified.
    #[serde(rename = "modelspec.unet_dtype")]
    pub unet_dtype: Option<String>,

    /// (Specialty) for latent models with a VAE that has special DType requirements
    /// (eg incompatible with fp16 but works with bf16) for inference,
    /// a comma-separated list of known-good types. Inference engines are
    /// recommended to ensure a compatible type is used when this is specified.
    #[serde(rename = "modelspec.vae_dtype")]
    pub vae_dtype: Option<String>,

    // Text-Prediction Models
    /// The format the data is in - needed due to the variety of specialty formats
    /// and quantization methods (often not accurately reflected in tensor data type,
    /// as eg there are different definitions of 4bit data).
    #[serde(rename = "modelspec.data_format")]
    pub data_format: Option<String>,

    /// What `type of format` the model is intended to work in (writing stories vs
    /// question-and-answer chat vs coding). Should constrain to the enumerated
    /// examples unless a new format type has been created that is not yet listed.
    #[serde(rename = "modelspec.format_type")]
    pub format_type: Option<String>,

    /// The primary human language(s) the model is trained to understand, in standard
    /// language code format, as a comma-separated list.
    #[serde(rename = "modelspec.language")]
    pub language: Option<String>,

    /// For formats where a specific template is trained in, it should be given here,
    /// as a string that identifies %%SYSTEM%%, %%USER%%, and %%AI%%.
    /// Some templates may exclude 'system' or add additional keys. Inferencing tools
    /// are encouraged to be lenient if the format does not match expectations.
    #[serde(rename = "modelspec.format_template")]
    pub format_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// The tensor format of the model.
    pub format: Option<TensorFormart>,

    /// Stability AI Model Metadata Standard.
    #[serde(flatten)]
    pub model_spec: Option<ModelSpec>,

    /// Other metadata information.
    #[serde(flatten)]
    pub others: HashMap<String, String>,
}
