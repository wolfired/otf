#![allow(dead_code)]

use std::error::Error;
use std::io::Read;

use super::types::*;

pub fn read_uint8<R: Read>(r: &mut R) -> Result<u8, Box<dyn Error>> {
    let mut bs = [0];
    r.read(&mut bs)?;
    Ok(bs[0])
}

pub fn read_int8<R: Read>(r: &mut R) -> Result<i8, Box<dyn Error>> {
    let mut bs = [0];
    r.read(&mut bs)?;
    Ok(i8::from_be_bytes(bs))
}

pub fn read_uint16<R: Read>(r: &mut R) -> Result<u16, Box<dyn Error>> {
    let mut bs = [0, 0];
    r.read(&mut bs)?;
    Ok(u16::from_be_bytes(bs))
}

pub fn read_int16<R: Read>(r: &mut R) -> Result<i16, Box<dyn Error>> {
    let mut bs = [0, 0];
    r.read(&mut bs)?;
    Ok(i16::from_be_bytes(bs))
}

pub fn read_uint24<R: Read>(r: &mut R) -> Result<u32, Box<dyn Error>> {
    let mut bs = [0, 0, 0, 0];
    r.read(&mut bs[1..4])?;
    Ok(u32::from_be_bytes(bs))
}

pub fn read_uint32<R: Read>(r: &mut R) -> Result<u32, Box<dyn Error>> {
    let mut bs = [0, 0, 0, 0];
    r.read(&mut bs)?;
    Ok(u32::from_be_bytes(bs))
}

pub fn read_int32<R: Read>(r: &mut R) -> Result<i32, Box<dyn Error>> {
    let mut bs = [0, 0, 0, 0];
    r.read(&mut bs)?;
    Ok(i32::from_be_bytes(bs))
}

pub fn read_tag<R: Read>(r: &mut R) -> Result<Tag, Box<dyn Error>> {
    let mut bs = [0, 0, 0, 0];
    r.read(&mut bs)?;
    Ok(Tag(bs))
}

#[inline]
pub fn read_offset8<R: Read>(r: &mut R) -> Result<Offset8, Box<dyn Error>> {
    read_uint8(r)
}

#[inline]
pub fn read_offset16<R: Read>(r: &mut R) -> Result<Offset16, Box<dyn Error>> {
    read_uint16(r)
}

#[inline]
pub fn read_offset24<R: Read>(r: &mut R) -> Result<Offset24, Box<dyn Error>> {
    read_uint24(r)
}

#[inline]
pub fn read_offset32<R: Read>(r: &mut R) -> Result<Offset32, Box<dyn Error>> {
    read_uint32(r)
}

pub fn read_version16dot16<R: Read>(r: &mut R) -> Result<Version16Dot16, Box<dyn Error>> {
    let mut bs = [0, 0, 0, 0];
    r.read(&mut bs)?;
    Ok(Version16Dot16([(bs[0] as u16) << 8 + bs[1], (bs[2] as u16) << 8 + bs[3]]))
}

pub fn read_fixed<R: Read>(r: &mut R) -> Result<Fixed, Box<dyn Error>> {
    let i = read_int32(r)?;
    Ok(i as f32 / (1 << 16) as f32)
}

pub fn read_longdatetime<R: Read>(r: &mut R) -> Result<LongDateTime, Box<dyn Error>> {
    let mut bs = [0, 0, 0, 0, 0, 0, 0, 0];
    r.read(&mut bs)?;
    Ok(LongDateTime(i64::from_be_bytes(bs)))
}

pub fn get_platform_desc(platform_id: u16) -> &'static str {
    match platform_id {
        0 => "Unicode",
        1 => "Macintosh",
        2 => "ISO [deprecated]",
        3 => "Windows",
        4 => "Custom",
        _ => "error platform id",
    }
}

pub fn get_encoding_desc(platform_id: u16, encoding_id: u16) -> &'static str {
    match platform_id {
        0 => match encoding_id {
            0 => "Unicode 1.0 semantics—deprecated",
            1 => "Unicode 1.1 semantics—deprecated",
            2 => "ISO/IEC 10646 semantics—deprecated",
            3 => "Unicode 2.0 and onwards semantics, Unicode BMP only",
            4 => "Unicode 2.0 and onwards semantics, Unicode full repertoire",
            5 => "Unicode variation sequences—for use with subtable format 14",
            6 => "Unicode full repertoire—for use with subtable format 13",
            _ => "error encoding id",
        },
        1 => match encoding_id {
            0 => "Roman",
            1 => "Japanese",
            2 => "Chinese (Traditional)",
            3 => "Korean",
            4 => "Arabic",
            5 => "Hebrew",
            6 => "Greek",
            7 => "Russian",
            8 => "RSymbol",
            9 => "Devanagari",
            10 => "Gurmukhi",
            11 => "Gujarati",
            12 => "Odia",
            13 => "Bangla",
            14 => "Tamil",
            15 => "Telugu",
            16 => "Kannada",
            17 => "Malayalam",
            18 => "Sinhalese",
            19 => "Burmese",
            20 => "Khmer",
            21 => "Thai",
            22 => "Laotian",
            23 => "Georgian",
            24 => "Armenian",
            25 => "Chinese (Simplified)",
            26 => "Tibetan",
            27 => "Mongolian",
            28 => "Geez",
            29 => "Slavic",
            30 => "Vietnamese",
            31 => "Sindhi",
            32 => "Uninterpreted",
            _ => "error encoding id",
        },
        3 => match encoding_id {
            0 => "Symbol",
            1 => "Unicode BMP",
            2 => "ShiftJIS",
            3 => "PRC",
            4 => "Big5",
            5 => "Wansung",
            6 => "Johab",
            7 => "Reserved",
            8 => "Reserved",
            9 => "Reserved",
            10 => "Unicode full repertoire",
            _ => "error encoding id",
        },
        _ => "error platform id",
    }
}

pub fn get_language_desc(platform_id: u16, language_id: u16) -> &'static str {
    match platform_id {
        0 => "None",
        1 => match language_id {
            0 => "English",
            1 => "French",
            2 => "German",
            3 => "Italian",
            4 => "Dutch",
            5 => "Swedish",
            6 => "Spanish",
            11 => "Japanese",
            19 => "Chinese (traditional)",
            23 => "Korean",
            32 => "Russian",
            33 => "Chinese (simplified)",
            51 => "Armenian",
            _ => "error language id",
        },
        3 => match language_id {
            0x0009 => "English(en)",
            0x0409 => "English(en-US)",
            0x0004 => "Chinese (Simplified)(zh-Hans)",
            0x7804 => "Chinese (Simplified)(zh)",
            0x0804 => "Chinese (Simplified)(zh-CN)",
            0x1004 => "Chinese (Simplified)(zh-SG)",
            0x7C04 => "Chinese (Traditional)(zh-Hant)",
            0x0C04 => "Chinese (Traditional)(zh-HK)",
            0x1404 => "Chinese (Traditional)(zh-MO)",
            0x0404 => "Chinese (Traditional)(zh-TW)",
            _ => "error language id",
        },
        _ => "error platform id",
    }
}
