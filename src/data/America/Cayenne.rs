
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Cayenne",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -9040,  // UTC offset -9040, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1846272560, FixedTimespan {  // 1911-06-01T2-30-40 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GFT"),
        }),
        (-71092800, FixedTimespan {  // 1967-09-01T4-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GFT"),
        }),
    ]},
};


