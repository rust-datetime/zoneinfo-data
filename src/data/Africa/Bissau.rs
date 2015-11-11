
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Africa/Bissau",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -3460,  // UTC offset -3460, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-1830380540, FixedTimespan {  // 1912-00-01T0-57-40 UTC
            offset: -3600,  // UTC offset -3600, DST offset 0
            is_dst: false,
            name:   "WAT",
        }),
        (157770000, FixedTimespan {  // 1975-00-01T1-00-00 UTC
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   "GMT",
        }),
    ]},
};


