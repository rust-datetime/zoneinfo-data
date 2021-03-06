
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Galapagos",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -14496,  // UTC offset -14496, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1230753504, FixedTimespan {  // 1931-01-01T04:01:36.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ECT"),
        }),
        (504939600, FixedTimespan {  // 1986-01-01T05:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GALT"),
        }),
    ]},
};


