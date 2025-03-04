use std::fmt::Debug;

use chrono::Local;
use chrono::TimeDelta;
use chrono::TimeZone;
use chrono::Utc;

pub type Offset8 = u8;
pub type Offset16 = u16;
pub type Offset24 = u32;
pub type Offset32 = u32;
/// 32-bit signed fixed-point number (16.16)
pub type Fixed = f32;

pub struct Tag(pub(super) [u8; 4]);

impl Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Tag").field_with(|f| write!(f, "{}", unsafe { str::from_utf8_unchecked(&self.0) })).finish()
    }
}

impl PartialEq<[u8; 4]> for Tag {
    fn eq(&self, other: &[u8; 4]) -> bool {
        self.0.eq(other)
    }
}

pub struct Version16Dot16(pub(super) [u16; 2]);

impl Debug for Version16Dot16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Version16Dot16").field(&self.0).finish()
    }
}

/// Date and time represented in number of seconds since 12:00 midnight, January 1, 1904, UTC. The value is represented as a signed 64-bit integer.
pub struct LongDateTime(pub(super) i64);

impl Debug for LongDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("LongDateTime")
            .field_with(|f| write!(f, "{}", Utc.with_ymd_and_hms(1904, 1, 1, 0, 0, 0).unwrap().checked_add_signed(TimeDelta::seconds(self.0)).unwrap().with_timezone(&Local)))
            .finish()
    }
}
