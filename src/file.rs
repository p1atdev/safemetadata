use std::{
    fs::File,
    io::{self, BufWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::parser::{LocalParser, MetadataParser};

#[derive(Debug, Clone)]
pub struct SafetensorsFile {
    file_path: PathBuf,
    parser: LocalParser,
}

impl SafetensorsFile {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        let parser = LocalParser::new(file_path.as_ref());

        Self {
            file_path: file_path.as_ref().to_path_buf(),
            parser: parser,
        }
    }

    pub fn clear_metadata<P: AsRef<Path>>(&self, output_path: P) -> Result<()> {
        let output_file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output_path)?;

        let mut writer = BufWriter::new(output_file);

        let header_buffer_size = self.parser.get_header_size()?;

        // remove metadata
        let mut header = self.parser.parse_header()?;
        if let Some(metadata) = header.metadata {
            let bytes = serde_json::to_vec(&metadata)?.len() as u64;
            println!("Found metadata: {} bytes", bytes);
        } else {
            println!("No metadata found");
        }

        header.metadata = None;

        let new_header_buffer = serde_json::to_vec(&header)?;
        let new_header_size = new_header_buffer.len() as u64; // smaller

        // first part: header size [u8; 8]
        writer.write_all(&new_header_size.to_le_bytes())?;

        // second part: header buffer
        writer.write_all(&new_header_buffer)?;
        writer.flush()?;

        // third part: weights
        // move the original third part to the new position
        let mut source_file = File::open(&self.file_path)?;
        // skip the header size and the header buffer
        source_file.seek(SeekFrom::Start(8 + header_buffer_size))?;
        let mut dst_file = File::options()
            .write(true)
            .create(false)
            .append(true)
            .truncate(false)
            .open(&output_path)?;
        io::copy(&mut source_file, &mut dst_file)?;

        Ok(())
    }
}
