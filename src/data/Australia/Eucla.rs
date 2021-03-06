
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Australia/Eucla",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 30928,  // UTC offset 30928, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2337928528, FixedTimespan {  // 1895-11-30T15:24:32.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
        (-1672562640, FixedTimespan {  // 1916-12-31T15:16:00.000 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACWDT"),
        }),
        (-1665387900, FixedTimespan {  // 1917-03-24T16:15:00.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
        (-883637100, FixedTimespan {  // 1941-12-31T17:15:00.000 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACWDT"),
        }),
        (-876123900, FixedTimespan {  // 1942-03-28T16:15:00.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
        (-860395500, FixedTimespan {  // 1942-09-26T17:15:00.000 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACWDT"),
        }),
        (-844674300, FixedTimespan {  // 1943-03-27T16:15:00.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
        (152039700, FixedTimespan {  // 1974-10-26T17:15:00.000 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACWDT"),
        }),
        (162926100, FixedTimespan {  // 1975-03-01T17:15:00.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
        (436295700, FixedTimespan {  // 1983-10-29T17:15:00.000 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACWDT"),
        }),
        (447182100, FixedTimespan {  // 1984-03-03T17:15:00.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
        (690311700, FixedTimespan {  // 1991-11-16T17:15:00.000 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACWDT"),
        }),
        (699470100, FixedTimespan {  // 1992-03-01T17:15:00.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
        (1165079700, FixedTimespan {  // 2006-12-02T17:15:00.000 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACWDT"),
        }),
        (1174756500, FixedTimespan {  // 2007-03-24T17:15:00.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
        (1193505300, FixedTimespan {  // 2007-10-27T17:15:00.000 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACWDT"),
        }),
        (1206810900, FixedTimespan {  // 2008-03-29T17:15:00.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
        (1224954900, FixedTimespan {  // 2008-10-25T17:15:00.000 UTC
            offset: 35100,  // UTC offset 31500, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACWDT"),
        }),
        (1238260500, FixedTimespan {  // 2009-03-28T17:15:00.000 UTC
            offset: 31500,  // UTC offset 31500, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACWST"),
        }),
    ]},
};


