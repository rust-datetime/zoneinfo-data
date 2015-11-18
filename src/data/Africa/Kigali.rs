
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Africa/Kigali",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 7820,  // UTC offset 7820, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2109291020, FixedTimespan {  // 1903-01-28T21-49-40 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CAT"),
        }),
    ]},
};


