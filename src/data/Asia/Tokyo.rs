
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Asia/Tokyo",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 33539,  // UTC offset 33539, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2587745939, FixedTimespan {  // 1887-11-31T5-41-01 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JST",
        }),
        (-2335251600, FixedTimespan {  // 1895-11-31T15-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JCST",
        }),
        (-1017824400, FixedTimespan {  // 1937-08-30T15-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JST",
        }),
        (-683794800, FixedTimespan {  // 1948-04-01T17-00-00 UTC
            offset: 36000,  // UTC offset 32400, DST offset 3600
            is_dst: true,
            name:   "JDT",
        }),
        (-672393600, FixedTimespan {  // 1948-08-10T16-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JST",
        }),
        (-654764400, FixedTimespan {  // 1949-03-02T17-00-00 UTC
            offset: 36000,  // UTC offset 32400, DST offset 3600
            is_dst: true,
            name:   "JDT",
        }),
        (-640944000, FixedTimespan {  // 1949-08-09T16-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JST",
        }),
        (-620290800, FixedTimespan {  // 1950-04-06T17-00-00 UTC
            offset: 36000,  // UTC offset 32400, DST offset 3600
            is_dst: true,
            name:   "JDT",
        }),
        (-609494400, FixedTimespan {  // 1950-08-08T16-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JST",
        }),
        (-588841200, FixedTimespan {  // 1951-04-05T17-00-00 UTC
            offset: 36000,  // UTC offset 32400, DST offset 3600
            is_dst: true,
            name:   "JDT",
        }),
        (-578044800, FixedTimespan {  // 1951-08-07T16-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JST",
        }),
    ]},
};


