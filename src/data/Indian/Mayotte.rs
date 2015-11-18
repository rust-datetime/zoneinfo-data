
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Indian/Mayotte",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 8836,  // UTC offset 8836, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1309746436, FixedTimespan {  // 1928-05-30T21-32-44 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAT"),
        }),
        (-1262314800, FixedTimespan {  // 1929-11-31T21-00-00 UTC
            offset: 9000,  // UTC offset 9000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BEAT"),
        }),
        (-946780200, FixedTimespan {  // 1939-11-31T21-30-00 UTC
            offset: 9900,  // UTC offset 9900, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BEAUT"),
        }),
        (-315629100, FixedTimespan {  // 1959-11-31T21-15-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAT"),
        }),
    ]},
};


