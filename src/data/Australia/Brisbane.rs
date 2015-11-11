
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Australia/Brisbane",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 36728,  // UTC offset 36728, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2366791928, FixedTimespan {  // 1894-11-31T13-47-52 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (-1672567140, FixedTimespan {  // 1916-11-31T14-01-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (-1665392400, FixedTimespan {  // 1917-02-24T15-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (-883641600, FixedTimespan {  // 1941-11-31T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (-876128400, FixedTimespan {  // 1942-02-28T15-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (-860400000, FixedTimespan {  // 1942-08-26T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (-844678800, FixedTimespan {  // 1943-02-27T15-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (-828345600, FixedTimespan {  // 1943-09-02T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (-813229200, FixedTimespan {  // 1944-02-25T15-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (57686400, FixedTimespan {  // 1971-09-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (67968000, FixedTimespan {  // 1972-01-26T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (625593600, FixedTimespan {  // 1989-09-28T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (636480000, FixedTimespan {  // 1990-02-03T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (657043200, FixedTimespan {  // 1990-09-27T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (667929600, FixedTimespan {  // 1991-02-02T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (688492800, FixedTimespan {  // 1991-09-26T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (699465600, FixedTimespan {  // 1992-02-01T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
    ]},
};


