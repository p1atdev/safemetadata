# safemetadata

> [!WARNING]
> WIP

A CLI tool for viewing metadata of [🤗safetensors](https://github.com/huggingface/safetensors) files.

- Supports viewing model parameters, layers and modelspec.
- Supports remote files on Hugging Face Hub without full model download. ([See details](https://huggingface.co/docs/safetensors/metadata_parsing))

## Installation

```bash
cargo install --git https://github.com/p1atdev/safemetadata
```

## Usage

### Param size

```bash
safemtetadata params ./Qwen2-0.5B-Instruct/model.safetensors # local file
```

or

```bash
safemtetadata params model.safetensors --repo-id Qwen/Qwen2-0.5B-Instruct # on huggingface hub
```

Output:


```
Total parameters: 0.5B params
```

### Layers

```bash
safemtetadata layers ./Qwen2-0.5B-Instruct/model.safetensors
```

or

```bash
safemtetadata layers model.safetensors --repo-id Qwen/Qwen2-0.5B-Instruct
```

Output:

```
Tensor format: PyTorch
╭────────────────────────────────────────────┬──────────┬───────────────╮
│ Parameter Name                             │ DType    │ Shape         │
├────────────────────────────────────────────┼──────────┼───────────────┤
│ model.embed_tokens.weight                  │ bfloat16 │ [151936, 896] │
│ model.layers.0.input_layernorm.weight      │ bfloat16 │ [896]         │
│ model.layers.0.mlp.down_proj.weight        │ bfloat16 │ [896, 4864]   │
│ model.layers.0.mlp.gate_proj.weight        │ bfloat16 │ [4864, 896]   │
│ model.layers.0.mlp.up_proj.weight          │ bfloat16 │ [4864, 896]   │
│ model.layers.0.post_attention_layernorm.we │ bfloat16 │ [896]         │
│ ight                                       │          │               │
│ model.layers.0.self_attn.k_proj.bias       │ bfloat16 │ [128]         │
...
```

### SAI ModelSpec

```bash
safemtetadata modelspec ./sd_xl_base_1.0_0.9vae.safetensors
```

or

```bash
safemtetadata modelspec sd_xl_base_1.0_0.9vae.safetensors --repo-id stabilityai/stable-diffusion-xl-base-1.0
```

Output:

```
Stability AI Model Metadata Standard Specification
╭───────────────────────────┬────────────────────────────────────────────╮
│ Key                       │ Value                                      │
├───────────────────────────┼────────────────────────────────────────────┤
│ modelspec.architecture    │ "stable-diffusion-xl-v1-base"              │
│ modelspec.author          │ "StabilityAI"                              │
│ modelspec.date            │ "2023-07-26"                               │
│ modelspec.description     │ "SDXL 1.0 Base Model, compositional expert │
│                           │ . SDXL, the most advanced development in t │
│                           │ he Stable Diffusion text-to-image suite of │
│                           │  models. SDXL produces massively improved  │
│                           │ image and composition detail over its pred │
│                           │ ecessors. The ability to generate hyper-re │
│                           │ alistic creations for films, television, m │
│                           │ usic, and instructional videos, as well as │
│                           │  offering advancements for design and indu │
│                           │ strial use, places SDXL at the forefront o │
│                           │ f real world applications for AI imagery." │
│ modelspec.hash_sha256     │ "0x5e756477ea9ddde7a552c6e2d7926f849e8b0df │
│                           │ d2f0b513ff17b7d31faedd79f"                 │
│ modelspec.implementation  │ "https://github.com/Stability-AI/generativ │
│                           │ e-models"                                  │
│ modelspec.license         │ "CreativeML Open RAIL++-M License"         │
│ modelspec.prediction_type │ "epsilon"                                  │
│ modelspec.resolution      │ "1024x1024"                                │
│ modelspec.sai_model_spec  │ "1.0.0"                                    │
...
```