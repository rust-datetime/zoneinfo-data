
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Pacific/Bougainville",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 37336,  // UTC offset 37336, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2840178136, FixedTimespan {  // 1879-11-31T13-37-44 UTC
            offset: 35312,  // UTC offset 35312, DST offset 0
            is_dst: false,
            name:   "PMMT",
        }),
        (-2366790512, FixedTimespan {  // 1894-11-31T14-11-28 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "PGT",
        }),
        (-868010400, FixedTimespan {  // 1942-05-30T14-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JST",
        }),
        (-768906000, FixedTimespan {  // 1945-07-20T15-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "PGT",
        }),
        (1419696000, FixedTimespan {  // 2014-11-27T16-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "BST",
        }),
    ]},
};


