---
name: safemetadata-cli
description: Run the safemetadata Rust CLI from this project at a fixed Git commit. Use when Codex needs to inspect safetensors parameter counts, layer tables, metadata, Stability AI ModelSpec fields, or remove metadata through the bundled cargo-install wrapper.
---

# Safemetadata CLI

## Overview

Use this skill to call the `safemetadata` CLI through `scripts/safemetadata`. The wrapper installs and executes the binary from a fixed Git revision, so do not call a system `safemetadata` binary directly.

## Wrapper

Run the wrapper from this skill directory:

```bash
scripts/safemetadata <subcommand> <args>
```

The script installs with this fixed source:

```bash
cargo install --git https://github.com/p1atdev/safemetadata.git --rev <REVISION> --locked --bin safemetadata --root <cache-root> safemetadata
```

By default, `<cache-root>` is `$HOME/.cache/codex/safemetadata-cli/<REVISION>`. Set `SAFEMETADATA_CLI_ROOT` only when the user explicitly wants a different install root.

If the cached binary exists without the matching revision marker, stop and ask whether to remove or reinstall the cache. Do not run another `safemetadata` binary or build from the local checkout.

## Commands

- `params <file> [--repo-id <org/model>] [--token <token>]`: Show total parameter count.
- `layers <file> [--repo-id <org/model>] [--token <token>]`: Show tensor format and layer table.
- `modelspec <file> [--repo-id <org/model>] [--token <token>]`: Show Stability AI ModelSpec fields.
- `metadata <file> [--repo-id <org/model>] [--token <token>]`: Show all safetensors `__metadata__` fields.
- `clean <local-file> --output <output-file>`: Remove metadata from a local safetensors file.

## Workflow

1. Resolve this skill directory and invoke `scripts/safemetadata`.
2. For Hugging Face Hub files, pass the file path within the repo plus `--repo-id <org/model>`.
3. Pass `--token` only when the user explicitly provides a Hugging Face token.
4. For `clean`, require an explicit output path before running the command.
5. If `cargo` is unavailable, the network is unavailable, or installation fails, report the exact failure and stop.

## Examples

```bash
scripts/safemetadata params model.safetensors --repo-id Qwen/Qwen2-0.5B-Instruct
scripts/safemetadata layers ./model.safetensors
scripts/safemetadata modelspec sd_xl_base_1.0_0.9vae.safetensors --repo-id stabilityai/stable-diffusion-xl-base-1.0
scripts/safemetadata metadata ./model.safetensors
scripts/safemetadata clean ./model.safetensors --output ./model-cleaned.safetensors
```

Use `scripts/safemetadata --revision` when you need to confirm the fixed commit without installing.
