use std::str::FromStr;

mod error;

mod gzip;
mod snappy;
mod lz4;

pub use error::CompressionError;
use serde::{Serialize, Deserialize};

/// The compression algorithm used to compress and decompress records in fluvio batches
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[repr(i8)]
pub enum Compression {
    None = 0,
    Gzip = 1,
    Snappy = 2,
    Lz4 = 3,
}

impl Default for Compression {
    fn default() -> Self {
        Compression::None
    }
}

impl TryFrom<i8> for Compression {
    type Error = CompressionError;
    fn try_from(v: i8) -> Result<Self, CompressionError> {
        match v {
            0 => Ok(Compression::None),
            1 => Ok(Compression::Gzip),
            2 => Ok(Compression::Snappy),
            3 => Ok(Compression::Lz4),
            _ => Err(CompressionError::UnknownCompressionFormat(format!(
                "i8 representation: {}",
                v
            ))),
        }
    }
}

impl FromStr for Compression {
    type Err = CompressionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Compression::None),
            "gzip" => Ok(Compression::Gzip),
            "snappy" => Ok(Compression::Snappy),
            "lz4" => Ok(Compression::Lz4),
            _ => Err(CompressionError::UnknownCompressionFormat(s.into())),
        }
    }
}

impl Compression {
    /// Compress the given data, returning the compressed data
    pub fn compress(&self, src: &[u8]) -> Result<Vec<u8>, CompressionError> {
        match *self {
            Compression::None => Ok(src.to_vec()),
            Compression::Gzip => gzip::compress(src),
            Compression::Snappy => snappy::compress(src),
            Compression::Lz4 => lz4::compress(src),
        }
    }

    /// Uncompresss the given data, returning the uncompressed data if any compression was applied, otherwise returns None
    pub fn uncompress(&self, src: &[u8]) -> Result<Option<Vec<u8>>, CompressionError> {
        match *self {
            Compression::None => Ok(None),
            Compression::Gzip => {
                let output = gzip::uncompress(src)?;
                Ok(Some(output))
            }
            Compression::Snappy => {
                let output = snappy::uncompress(src)?;
                Ok(Some(output))
            }
            Compression::Lz4 => {
                let output = lz4::uncompress(src)?;
                Ok(Some(output))
            }
        }
    }
}