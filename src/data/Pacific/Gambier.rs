
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Gambier",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -25212,  // UTC offset -25212, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1806685188, FixedTimespan {  // 1912-09-01T7-00-12 UTC
            offset: -32400,  // UTC offset -32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GAMT"),
        }),
    ]},
};


