
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Australia/Eucla",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 30928,  // UTC offset 30928, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2337928528, FixedTimespan {  // 1895-10-30T15-24-32 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
        (-1672562640, FixedTimespan {  // 1916-11-31T15-16-00 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   "ACWDT",
        }),
        (-1665387900, FixedTimespan {  // 1917-02-24T16-15-00 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
        (-883637100, FixedTimespan {  // 1941-11-31T17-15-00 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   "ACWDT",
        }),
        (-876123900, FixedTimespan {  // 1942-02-28T16-15-00 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
        (-860395500, FixedTimespan {  // 1942-08-26T17-15-00 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   "ACWDT",
        }),
        (-844674300, FixedTimespan {  // 1943-02-27T16-15-00 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
        (152039700, FixedTimespan {  // 1974-09-26T17-15-00 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   "ACWDT",
        }),
        (162926100, FixedTimespan {  // 1975-02-01T17-15-00 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
        (436295700, FixedTimespan {  // 1983-09-29T17-15-00 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   "ACWDT",
        }),
        (447182100, FixedTimespan {  // 1984-02-03T17-15-00 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
        (690311700, FixedTimespan {  // 1991-10-16T17-15-00 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   "ACWDT",
        }),
        (699470100, FixedTimespan {  // 1992-02-01T17-15-00 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
        (1165079700, FixedTimespan {  // 2006-11-02T17-15-00 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   "ACWDT",
        }),
        (1174756500, FixedTimespan {  // 2007-02-24T17-15-00 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
        (1193505300, FixedTimespan {  // 2007-09-27T17-15-00 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   "ACWDT",
        }),
        (1206810900, FixedTimespan {  // 2008-02-29T17-15-00 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
        (1224954900, FixedTimespan {  // 2008-09-25T17-15-00 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   "ACWDT",
        }),
        (1238260500, FixedTimespan {  // 2009-02-28T17-15-00 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   "ACWST",
        }),
    ]},
};


