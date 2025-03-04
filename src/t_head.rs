use std::error::Error;
use std::fmt::Debug;

use super::types::*;
use super::utils::*;

pub struct FontHeaderTable {
    major_version: u16,
    minor_version: u16,
    font_revision: Fixed,
    checksum_adjustment: u32,
    magic_number: u32,
    flags: u16,
    units_per_em: u16,
    created: LongDateTime,
    modified: LongDateTime,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    index_to_loc_format: i16,
    glyph_data_format: i16,
}

impl Debug for FontHeaderTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FontHeaderTable")
            .field("major_version", &self.major_version)
            .field("minor_version", &self.minor_version)
            .field("font_revision", &self.font_revision)
            .field("checksum_adjustment", &self.checksum_adjustment)
            .field_with("magic_number", |f| {
                write!(f, "0x{:08x}", &self.magic_number)
            })
            .field("flags", &self.flags)
            .field("units_per_em", &self.units_per_em)
            .field("created", &self.created)
            .field("modified", &self.modified)
            .field("x_min", &self.x_min)
            .field("y_min", &self.y_min)
            .field("x_max", &self.x_max)
            .field("y_max", &self.y_max)
            .field("mac_style", &self.mac_style)
            .field("lowest_rec_ppem", &self.lowest_rec_ppem)
            .field("font_direction_hint", &self.font_direction_hint)
            .field("index_to_loc_format", &self.index_to_loc_format)
            .field("glyph_data_format", &self.glyph_data_format)
            .finish()
    }
}

impl FontHeaderTable {
    pub fn from_bytes(mut b: &[u8]) -> Result<Self, Box<dyn Error>> {
        let r = &mut b;

        let major_version = read_uint16(r)?;
        let minor_version = read_uint16(r)?;
        let font_revision = read_fixed(r)?;
        let checksum_adjustment = read_uint32(r)?;
        let magic_number = read_uint32(r)?;
        let flags = read_uint16(r)?;
        let units_per_em = read_uint16(r)?;
        let created = read_longdatetime(r)?;
        let modified = read_longdatetime(r)?;
        let x_min = read_int16(r)?;
        let y_min = read_int16(r)?;
        let x_max = read_int16(r)?;
        let y_max = read_int16(r)?;
        let mac_style = read_uint16(r)?;
        let lowest_rec_ppem = read_uint16(r)?;
        let font_direction_hint = read_int16(r)?;
        let index_to_loc_format = read_int16(r)?;
        let glyph_data_format = read_int16(r)?;

        Ok(Self {
            major_version,
            minor_version,
            font_revision,
            checksum_adjustment,
            magic_number,
            flags,
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem,
            font_direction_hint,
            index_to_loc_format,
            glyph_data_format,
        })
    }
}
