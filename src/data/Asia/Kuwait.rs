
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Kuwait",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 11212,  // UTC offset 11212, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-719636812, FixedTimespan {  // 1947-02-13T20-53-08 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
    ]},
};


