
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Urumqi",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 21020,  // UTC offset 21020, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1325483420, FixedTimespan {  // 1927-11-31T18-09-40 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("XJT"),
        }),
    ]},
};


