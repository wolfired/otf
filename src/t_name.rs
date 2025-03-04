use std::error::Error;
use std::fmt::Debug;

use super::types::*;
use super::utils::*;

pub struct NamingTable {
    version: u16,
    count: u16,
    /// Offset to start of string storage (from start of table)
    storage_offset: Offset16,
    name_record: Vec<NameRecord>,
    lang_tag_count: Option<u16>,                 // ver 1
    lang_tag_record: Option<Vec<LangTagRecord>>, // ver 1
}

impl Debug for NamingTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NamingTable")
            .field("version", &self.version)
            .field("count", &self.count)
            .field("storage_offset", &self.storage_offset)
            .field("name_record", &self.name_record)
            .field("lang_tag_count", &self.lang_tag_count)
            .field("lang_tag_record", &self.lang_tag_record)
            .finish()
    }
}

impl NamingTable {
    pub fn from_bytes(mut b: &[u8]) -> Result<Self, Box<dyn Error>> {
        let o = b;
        let r = &mut b;

        let version = read_uint16(r)?;
        let count = read_uint16(r)?;
        let storage_offset = read_offset16(r)?;
        let mut name_record = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let mut nr = NameRecord::from_bytes(r)?;
            nr.content = Some(String::from_utf16be(&o[{ (storage_offset + nr.string_offset) as usize }..{ (storage_offset + nr.string_offset + nr.length) as usize }])?);
            name_record.push(nr);
        }

        if 0 == version {
            return Ok(Self {
                version,
                count,
                storage_offset,
                name_record,
                lang_tag_count: None,
                lang_tag_record: None,
            });
        }

        let lang_tag_count = read_uint16(r)?;
        let mut lang_tag_record = Vec::with_capacity(lang_tag_count as usize);

        for _ in 0..lang_tag_count {
            let mut ltr = LangTagRecord::from_bytes(r)?;
            ltr.content = Some(String::from_utf16be(&o[{ (storage_offset + ltr.lang_tag_offset) as usize }..{ (storage_offset + ltr.lang_tag_offset + ltr.length) as usize }])?);
            lang_tag_record.push(ltr);
        }

        return Ok(Self {
            version,
            count,
            storage_offset,
            name_record,
            lang_tag_count: Some(lang_tag_count),
            lang_tag_record: Some(lang_tag_record),
        });
    }
}

pub struct NameRecord {
    platform_id: u16,
    encoding_id: u16,
    language_id: u16,
    name_id: u16,
    length: u16,
    /// String offset from start of storage area (in bytes)
    string_offset: Offset16,
    content: Option<String>,
}

impl Debug for NameRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NameRecord")
            .field_with("platform_id", |f| write!(f, "{}({})", get_platform_desc(self.platform_id), &self.platform_id))
            .field_with("encoding_id", |f| write!(f, "{}({})", get_encoding_desc(self.platform_id, self.encoding_id), &self.encoding_id))
            .field_with("language_id", |f| write!(f, "{}({})", get_language_desc(self.platform_id, self.language_id), &self.language_id))
            .field_with("name_id", |f| write!(f, "{}({})", Self::get_name_desc(self.name_id), &self.name_id))
            .field("length", &self.length)
            .field("string_offset", &self.string_offset)
            .field("content", &self.content)
            .finish()
    }
}

impl NameRecord {
    pub fn from_bytes(r: &mut &[u8]) -> Result<Self, Box<dyn Error>> {
        let platform_id = read_uint16(r)?;
        let encoding_id = read_uint16(r)?;
        let language_id = read_uint16(r)?;
        let name_id = read_uint16(r)?;
        let length = read_uint16(r)?;
        let string_offset = read_offset16(r)?;

        Ok(Self {
            platform_id,
            encoding_id,
            language_id,
            name_id,
            length,
            string_offset,
            content: None,
        })
    }

    fn get_name_desc(name_id: u16) -> &'static str {
        match name_id {
            0 => "Copyright notice",
            1 => "Font Family name",
            2 => "Font Subfamily name",
            3 => "Unique font identifier",
            4 => "Full font name",
            5 => "Version string",
            6 => "PostScript name",
            7 => "Trademark",
            8 => "Manufacturer Name",
            9 => "Designer",
            10 => "Description",
            11 => "URL of Vendor",
            12 => "URL of Designer",
            13 => "License Description",
            14 => "License Info URL",
            15 => "Reserved",
            16 => "Typographic Family name",
            17 => "Typographic Subfamily name",
            18 => "Compatible Full (Macintosh only)",
            19 => "Sample text",
            20 => "PostScript CID findfont name",
            21 => "WWS Family Name",
            22 => "WWS Subfamily Name",
            23 => "Light Background Palette",
            24 => "Dark Background Palette",
            25 => "Variations PostScript Name Prefix",
            26..=255 => "reserved for future standard names",
            256..=32767 => "reserved for font-specific names",
            _ => "error name id",
        }
    }
}

pub struct LangTagRecord {
    length: u16,
    /// Language-tag string offset from start of storage area (in bytes).
    lang_tag_offset: Offset16,
    content: Option<String>,
}

impl LangTagRecord {
    pub fn from_bytes(r: &mut &[u8]) -> Result<Self, Box<dyn Error>> {
        let length = read_uint16(r)?;
        let lang_tag_offset = read_offset16(r)?;

        Ok(Self { length, lang_tag_offset, content: None })
    }
}

impl Debug for LangTagRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LangTagRecord").field("length", &self.length).field("lang_tag_offset", &self.lang_tag_offset).finish()
    }
}
