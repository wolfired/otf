use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::t_cmap::*;
use super::t_head::*;
use super::t_name::*;
use super::types::*;
use super::utils::*;

pub struct Font {
    table_directory: TableDirectory,
    t_name: Option<NamingTable>,
    t_cmap: Option<CharacterToGlyphIndexMappingTable>,
    t_head: Option<FontHeaderTable>,
}

impl Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Otf")
            .field("table_directory", &self.table_directory)
            .field("name", &self.t_name)
            .field("cmap", &self.t_cmap)
            .field("head", &self.t_head)
            .finish()
    }
}

impl Font {
    pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Self, Box<dyn Error>> {
        let mut f = File::open(p)?;
        let mut v = Vec::with_capacity(f.metadata()?.len() as usize);
        f.read_to_end(&mut v)?;

        let table_directory = TableDirectory::from_bytes(v.as_slice())?;

        let mut t_name = None;
        let mut t_cmap = None;
        let mut t_head = None;
        for tr in &table_directory.table_records {
            match &tr.table_tag.0 {
                b"name" => t_name = Some(NamingTable::from_bytes(&v.as_slice()[{ tr.offset as usize }..{ (tr.offset + tr.length) as usize }])?),
                b"cmap" => t_cmap = Some(CharacterToGlyphIndexMappingTable::from_bytes(&v.as_slice()[{ tr.offset as usize }..{ (tr.offset + tr.length) as usize }])?),
                b"head" => t_head = Some(FontHeaderTable::from_bytes(&v.as_slice()[{ tr.offset as usize }..{ (tr.offset + tr.length) as usize }])?),
                _ => {}
            }
        }

        Ok(Self { table_directory, t_name, t_cmap, t_head })
    }
}

pub struct TableDirectory {
    sfnt_version: u32,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    table_records: Vec<TableRecord>,
}

impl Debug for TableDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableDirectory")
            .field_with("sfnt_version", |f| write!(f, "0x{:08x}", &self.sfnt_version))
            .field("num_tables", &self.num_tables)
            .field("search_range", &self.search_range)
            .field("entry_selector", &self.entry_selector)
            .field("range_shift", &self.range_shift)
            .field("table_records", &self.table_records)
            .finish()
    }
}

impl TableDirectory {
    pub fn from_bytes(mut b: &[u8]) -> Result<Self, Box<dyn Error>> {
        let r = &mut b;

        let sfnt_version = read_uint32(r)?;
        let num_tables = read_uint16(r)?;
        let search_range = read_uint16(r)?;
        let entry_selector = read_uint16(r)?;
        let range_shift = read_uint16(r)?;

        let mut table_records = Vec::with_capacity(num_tables as usize);

        for _ in 0..num_tables {
            table_records.push(TableRecord::from_bytes(r)?);
        }

        Ok(Self {
            sfnt_version,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
            table_records,
        })
    }
}

pub struct TableRecord {
    table_tag: Tag,
    checksum: u32,
    /// Offset from beginning of font file
    offset: Offset32,
    length: u32,
}

impl Debug for TableRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableRecord")
            .field("table_tag", &self.table_tag)
            .field("checksum", &self.checksum)
            .field("offset", &self.offset)
            .field("length", &self.length)
            .finish()
    }
}

impl TableRecord {
    pub fn from_bytes(r: &mut &[u8]) -> Result<Self, Box<dyn Error>> {
        let table_tag = read_tag(r)?;
        let checksum = read_uint32(r)?;
        let offset = read_offset32(r)?;
        let length = read_uint32(r)?;

        Ok(Self { table_tag, checksum, offset, length })
    }
}
