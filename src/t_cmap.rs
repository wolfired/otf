use std::error::Error;
use std::fmt::Debug;

use super::types::*;
use super::utils::*;

pub struct CharacterToGlyphIndexMappingTable {
    version: u16,
    num_tables: u16,
    encoding_records: Vec<EncodingRecord>,
}

impl Debug for CharacterToGlyphIndexMappingTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CharacterToGlyphIndexMappingTable")
            .field("version", &self.version)
            .field("num_tables", &self.num_tables)
            .field("encoding_records", &self.encoding_records)
            .finish()
    }
}

impl CharacterToGlyphIndexMappingTable {
    pub fn from_bytes(mut b: &[u8]) -> Result<Self, Box<dyn Error>> {
        let o = b;
        let r = &mut b;

        let version = read_uint16(r)?;
        let num_tables = read_uint16(r)?;
        let mut encoding_records = Vec::with_capacity(num_tables as usize);

        for _ in 0..num_tables {
            let mut er = EncodingRecord::from_bytes(r)?;
            er.sub_table = Some(SubTable::from_bytes(&o[er.subtable_offset as usize..])?);
            encoding_records.push(er);
        }

        Ok(Self { version, num_tables, encoding_records })
    }
}

pub struct EncodingRecord {
    platform_id: u16,
    encoding_id: u16,
    /// Byte offset from beginning of table to the subtable for this encoding
    subtable_offset: Offset32,
    sub_table: Option<SubTable>,
}

impl Debug for EncodingRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncodingRecord")
            .field_with("platform_id", |f| write!(f, "{}({})", get_platform_desc(self.platform_id), &self.platform_id))
            .field_with("encoding_id", |f| write!(f, "{}({})", get_encoding_desc(self.platform_id, self.encoding_id), &self.encoding_id))
            .field("subtable_offset", &self.subtable_offset)
            .field("sub_table", &self.sub_table)
            .finish()
    }
}

impl EncodingRecord {
    pub fn from_bytes(r: &mut &[u8]) -> Result<Self, Box<dyn Error>> {
        let platform_id = read_uint16(r)?;
        let encoding_id = read_uint16(r)?;
        let subtable_offset = read_offset32(r)?;

        Ok(Self {
            platform_id,
            encoding_id,
            subtable_offset,
            sub_table: None,
        })
    }
}

pub struct SubTable {
    format: u16,
}

impl Debug for SubTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubTable").field_with("format", |f| write!(f, "{}({})", Self::get_format_desc(self.format), &self.format)).finish()
    }
}

impl SubTable {
    pub fn from_bytes(mut b: &[u8]) -> Result<Self, Box<dyn Error>> {
        let r = &mut b;

        let format = read_uint16(r)?;

        Ok(Self { format })
    }

    pub fn get_format_desc(format: u16) -> &'static str {
        match format {
            0 => "Byte encoding table",
            2 => "High byte mapping through table",
            4 => "Segment mapping to delta values",
            6 => "Trimmed table mapping",
            8 => "mixed 16-bit and 32-bit coverage",
            10 => "Trimmed array",
            12 => "Segmented coverage",
            13 => "Many-to-one range mappings",
            14 => "Unicode variation sequences",
            _ => "error format",
        }
    }
}

pub struct SubTable4 {
    length: u16,
    language: u16,
    
}
