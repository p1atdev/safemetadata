use super::fetch::fetch_remote_bytes;
use super::metadata::Header;
use anyhow::Result;
use hf_hub::api::sync::Api;
use hf_hub::{Repo, RepoType};
use std::io::{Read, Seek, SeekFrom};
use std::{fs::File, path::Path};

// ref: https://huggingface.co/docs/safetensors/index#format

/// Read the specified range of bytes of the safetensors file.
fn read_buffer<P: AsRef<Path>>(path: &P, start: u64, end: u64) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let length = (end - start) as usize; // read length
    let mut buffer = vec![0u8; length];

    file.seek(SeekFrom::Start(start))?;
    file.read_exact(&mut buffer)?;

    Ok(buffer.to_vec())
}

/// Metadata parser trait
pub trait MetadataParser {
    fn parse_header(&self) -> Result<Header>;
}

/// Read safetensors files from the local file system.
pub struct LocalParser<P: AsRef<Path>> {
    path: P,
}

impl<P: AsRef<Path>> LocalParser<P> {
    pub fn new(path: P) -> Self {
        Self { path: path }
    }

    /// Get the header size of the safetensors file.
    ///
    /// The header size chunk is the first 8 bytes of the file,
    /// which is the u64 int that represents the size of the header chunk.
    fn get_header_size(&self) -> Result<u64> {
        let buffer: [u8; 8] = read_buffer(&self.path, 0, 8)?.try_into().unwrap();

        // as usize
        let header_size = u64::from_le_bytes(buffer);

        Ok(header_size)
    }

    /// Read the header chunk of the safetensors file.
    ///
    /// The header chunk is a JSON string that contains the
    /// metadata of the file.
    fn get_header_buffer(&self, header_size: &u64) -> Result<Vec<u8>> {
        let buffer = read_buffer(&self.path, 8, 8 + header_size)?;

        Ok(buffer.to_vec())
    }
}

impl<P: AsRef<Path>> MetadataParser for LocalParser<P> {
    /// Parse the header of the safetensors file
    fn parse_header(&self) -> Result<Header> {
        let header_size = &self.get_header_size()?;
        let header_buffer = &self.get_header_buffer(&header_size)?;

        let header: Header = serde_json::from_slice(&header_buffer)?;

        Ok(header)
    }
}

#[cfg(test)]
mod test_local {
    use super::*;

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
    fn test_parse_header() {
        macro_rules! parse_header {
            ($repo_id:expr, $repo_type:expr, $filename:expr) => {
                let path = get_hub_file($repo_id, $repo_type, $filename).unwrap();

                let parser = LocalParser::new(path);
                let header = parser.parse_header();

                assert!(header.is_ok());
            };
        }

        parse_header!("p1atdev/dart-v2-sft", RepoType::Model, "model.safetensors");
        parse_header!(
            "p1atdev/wd-swinv2-tagger-v3-hf",
            RepoType::Model,
            "model.safetensors"
        );
        parse_header!(
            "runwayml/stable-diffusion-v1-5",
            RepoType::Model,
            "text_encoder/model.fp16.safetensors"
        );
        parse_header!(
            "Qwen/Qwen2-0.5B-Instruct",
            RepoType::Model,
            "model.safetensors"
        );
    }
}

pub struct RemoteParser {
    url: String,
    token: Option<String>,
}

impl RemoteParser {
    pub fn new(url: String) -> Self {
        Self {
            url: url,
            token: None,
        }
    }

    pub fn from_hub(
        repo_id: &str,
        repo_type: RepoType,
        filename: &str,
        token: &Option<String>,
    ) -> Self {
        let api = Api::new().unwrap();
        let repo = api.repo(Repo::new(repo_id.to_string(), repo_type));
        let url = repo.url(filename);

        Self {
            url: url.to_string(),
            token: token.clone(),
        }
    }
}

impl MetadataParser for RemoteParser {
    fn parse_header(&self) -> Result<Header> {
        let header_size_buffer = fetch_remote_bytes(&self.url, &self.token, 0, 8)?; // passing the start index and the length of bytes
        let header_size = u64::from_le_bytes(header_size_buffer.try_into().unwrap());

        let header_buffer = fetch_remote_bytes(&self.url, &self.token, 8, header_size)?;
        let header: Header = serde_json::from_slice(&header_buffer)?;

        Ok(header)
    }
}

#[cfg(test)]
mod test_remote {
    use super::*;

    #[test]
    fn test_parse_header_remote() {
        macro_rules! parse_header {
            ($repo_id:expr, $repo_type:expr, $filename:expr) => {
                let parser = RemoteParser::from_hub($repo_id, $repo_type, $filename, &None);

                let header = parser.parse_header();

                assert!(header.is_ok());
            };
        }

        parse_header!("p1atdev/dart-v2-sft", RepoType::Model, "model.safetensors");
        parse_header!(
            "p1atdev/wd-swinv2-tagger-v3-hf",
            RepoType::Model,
            "model.safetensors"
        );
        parse_header!(
            "runwayml/stable-diffusion-v1-5",
            RepoType::Model,
            "text_encoder/model.fp16.safetensors"
        );
        parse_header!(
            "Qwen/Qwen2-0.5B-Instruct",
            RepoType::Model,
            "model.safetensors"
        );
    }
}
