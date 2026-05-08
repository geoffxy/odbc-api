use odbc_sys::SmallInt;
use odbc_sys::USmallInt;
use odbc_sys::UInteger;

/// A padded version of `SQL_DATE_STRUCT` for alignment.
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DatePadded {
    pub year: SmallInt,   // 2 bytes
    pub month: USmallInt, // 2 bytes
    pub day: USmallInt,   // 2 bytes
    _padding: [u8; 2],    // 2 bytes
}

/// A padded version of `SQL_TIMESTAMP_STRUCT` for alignment.
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TimestampPadded {
    pub year: SmallInt,
    pub month: USmallInt,
    pub day: USmallInt,
    pub hour: USmallInt,
    pub minute: USmallInt,
    pub second: USmallInt,
    pub fraction: UInteger,
}
