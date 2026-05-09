use odbc_sys::SmallInt;
use odbc_sys::USmallInt;

/// A padded version of `SQL_DATE_STRUCT` to ensure 64-bit alignment.
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DatePadded {
    pub year: SmallInt,   // 2 bytes
    pub month: USmallInt, // 2 bytes
    pub day: USmallInt,   // 2 bytes
    _padding: [u8; 2],    // 2 bytes
}
