use super::metadata::Header;
use anyhow::Result;
use std::io::{Read, Seek, SeekFrom};
use std::{fs::File, path::Path};

// ref: https://huggingface.co/docs/safetensors/index#format

/// Read the specified range of bytes of the safetensors file.
fn read_buffer<P: AsRef<Path>>(path: P, start: u64, end: u64) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let length = (end - start) as usize; // read length
    let mut buffer = vec![0u8; length];

    file.seek(SeekFrom::Start(start))?;
    file.read_exact(&mut buffer)?;

    Ok(buffer.to_vec())
}

/// Get the header size of the safetensors file
fn get_header_size<P: AsRef<Path>>(path: P) -> Result<u64> {
    let buffer: [u8; 8] = read_buffer(path, 0, 8)?.try_into().unwrap();

    // as usize
    let header_size = u64::from_le_bytes(buffer);

    Ok(header_size)
}

/// Read the header of the safetensors file
fn get_header_bytes<P: AsRef<Path>>(path: P, header_size: &u64) -> Result<Vec<u8>> {
    let buffer = read_buffer(path, 8, 8 + header_size)?;

    Ok(buffer.to_vec())
}

/// Parse the header of the safetensors file
pub fn parse_header(path: &str) -> Result<Header> {
    let header_size = get_header_size(&path)?;

    let header_buffer = get_header_bytes(&path, &header_size)?;

    let header: Header = serde_json::from_slice(&header_buffer)?;

    Ok(header)
}

#[cfg(test)]
mod test {
    use super::*;

    use anyhow::Result;
    use hf_hub::api::sync::Api;
    use hf_hub::{Repo, RepoType};
    use std::path::PathBuf;

    fn get_hub_file(repo_id: &str, repo_type: RepoType, filename: &str) -> Result<PathBuf> {
        let api = Api::new()?;

        let repo = api.repo(Repo::new((&repo_id).to_string(), repo_type));

        let filepath = repo.get(filename).unwrap(); // if the cache is available, it will not download again

        Ok(filepath)
    }

    #[test]
    fn test_read_safetensors() {
        macro_rules! get_and_read_safetensors {
            ($repo_id:expr, $repo_type:expr, $filename:expr, $header_size_expected:expr) => {
                let path = get_hub_file($repo_id, $repo_type, $filename).unwrap();

                let header_size = get_header_size(&path).unwrap();

                assert_eq!(header_size, $header_size_expected);

                let header_buffer = get_header_bytes(&path, &header_size).unwrap();

                // convert to string
                let header_str = String::from_utf8(header_buffer).unwrap();

                assert_eq!(&header_str[..16], "{\"__metadata__\":");
            };
        }

        get_and_read_safetensors!(
            "p1atdev/dart-v2-sft",
            RepoType::Model,
            "model.safetensors",
            8352
        );
        get_and_read_safetensors!(
            "p1atdev/wd-swinv2-tagger-v3-hf",
            RepoType::Model,
            "model.safetensors",
            62576
        );
        get_and_read_safetensors!(
            "runwayml/stable-diffusion-v1-5",
            RepoType::Model,
            "text_encoder/model.fp16.safetensors",
            23280
        );
        get_and_read_safetensors!(
            "Qwen/Qwen2-0.5B-Instruct",
            RepoType::Model,
            "model.safetensors",
            32280
        );
    }

    #[test]
    fn test_parse_header() {
        macro_rules! parse_metadata {
            ($repo_id:expr, $repo_type:expr, $filename:expr) => {
                let path = get_hub_file($repo_id, $repo_type, $filename).unwrap();

                let header = parse_header(path.to_str().unwrap());

                assert!(header.is_ok());
            };
        }

        parse_metadata!("p1atdev/dart-v2-sft", RepoType::Model, "model.safetensors");
        parse_metadata!(
            "p1atdev/wd-swinv2-tagger-v3-hf",
            RepoType::Model,
            "model.safetensors"
        );
        parse_metadata!(
            "runwayml/stable-diffusion-v1-5",
            RepoType::Model,
            "text_encoder/model.fp16.safetensors"
        );
        parse_metadata!(
            "Qwen/Qwen2-0.5B-Instruct",
            RepoType::Model,
            "model.safetensors"
        );
    }
}
