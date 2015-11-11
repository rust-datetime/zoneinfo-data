
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Indian/Mahe",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 13308,  // UTC offset 13308, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2006653308, FixedTimespan {  // 1906-04-31T20-18-12 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "SCT",
        }),
    ]},
};


