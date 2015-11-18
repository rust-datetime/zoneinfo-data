
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Africa/Lagos",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 816,  // UTC offset 816, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1588464816, FixedTimespan {  // 1919-07-31T23-46-24 UTC
            offset: 3600,  // UTC offset 3600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WAT"),
        }),
    ]},
};


