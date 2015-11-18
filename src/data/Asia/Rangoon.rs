
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Rangoon",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 23080,  // UTC offset 23080, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2840163880, FixedTimespan {  // 1879-11-31T17-35-20 UTC
            offset: 23080,  // UTC offset 23080, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("RMT"),
        }),
        (-1577946280, FixedTimespan {  // 1919-11-31T17-35-20 UTC
            offset: 23400,  // UTC offset 23400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BURT"),
        }),
        (-873268200, FixedTimespan {  // 1942-03-30T17-30-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("JST"),
        }),
        (-778410000, FixedTimespan {  // 1945-04-02T15-00-00 UTC
            offset: 23400,  // UTC offset 23400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MMT"),
        }),
    ]},
};


