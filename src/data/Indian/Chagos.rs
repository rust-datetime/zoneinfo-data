
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Indian/Chagos",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 17380,  // UTC offset 17380, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1988167780, FixedTimespan {  // 1906-11-31T19-10-20 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IOT"),
        }),
        (820436400, FixedTimespan {  // 1995-11-31T19-00-00 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IOT"),
        }),
    ]},
};


