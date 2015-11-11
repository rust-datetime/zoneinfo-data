
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Pacific/Kiritimati",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -34240,  // UTC offset -34240, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2177418560, FixedTimespan {  // 1901-00-01T9-30-40 UTC
            offset: -33600,  // UTC offset -33600, DST offset 0
            is_dst: false,
            name:   "LINT",
        }),
        (307617600, FixedTimespan {  // 1979-09-01T9-20-00 UTC
            offset: -36000,  // UTC offset -36000, DST offset 0
            is_dst: false,
            name:   "LINT",
        }),
        (788954400, FixedTimespan {  // 1995-00-01T10-00-00 UTC
            offset: 50400,  // UTC offset 50400, DST offset 0
            is_dst: false,
            name:   "LINT",
        }),
    ]},
};


