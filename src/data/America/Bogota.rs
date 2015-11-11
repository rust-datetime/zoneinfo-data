
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "America/Bogota",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -11024,  // UTC offset -11024, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2707678576, FixedTimespan {  // 1884-02-13T3-03-44 UTC
            offset: -11024,  // UTC offset -11024, DST offset 0
            is_dst: false,
            name:   "BMT",
        }),
        (-1739048176, FixedTimespan {  // 1914-10-23T3-03-44 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   "COT",
        }),
        (704869200, FixedTimespan {  // 1992-04-03T5-00-00 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   "COST",
        }),
        (733896000, FixedTimespan {  // 1993-03-04T4-00-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   "COT",
        }),
    ]},
};


