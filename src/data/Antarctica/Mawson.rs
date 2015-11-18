
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Antarctica/Mawson",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("zzz"),
        },
        rest: &[
        (-501206400, FixedTimespan {  // 1954-01-13T0-00-00 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAWT"),
        }),
        (1255809600, FixedTimespan {  // 2009-09-17T20-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAWT"),
        }),
    ]},
};


