
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "America/Guatemala",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -21476,  // UTC offset -21476, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-1617040924, FixedTimespan {  // 1918-09-05T5-57-56 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (123055200, FixedTimespan {  // 1973-10-25T6-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   "CDT",
        }),
        (130914000, FixedTimespan {  // 1974-01-24T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (422344800, FixedTimespan {  // 1983-04-21T6-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   "CDT",
        }),
        (433054800, FixedTimespan {  // 1983-08-22T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (669708000, FixedTimespan {  // 1991-02-23T6-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   "CDT",
        }),
        (684219600, FixedTimespan {  // 1991-08-07T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (1146376800, FixedTimespan {  // 2006-03-30T6-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   "CDT",
        }),
        (1159678800, FixedTimespan {  // 2006-09-01T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
    ]},
};


