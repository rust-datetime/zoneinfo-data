
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "America/Porto_Velho",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -13464,  // UTC offset -13464, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-1767212136, FixedTimespan {  // 1914-00-01T3-44-24 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-1206954000, FixedTimespan {  // 1931-09-03T15-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-1191358800, FixedTimespan {  // 1932-03-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-1175371200, FixedTimespan {  // 1932-09-03T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-1159822800, FixedTimespan {  // 1933-03-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-633816000, FixedTimespan {  // 1949-11-01T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-622065600, FixedTimespan {  // 1950-03-16T4-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-602280000, FixedTimespan {  // 1950-11-01T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-591829200, FixedTimespan {  // 1951-03-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-570744000, FixedTimespan {  // 1951-11-01T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-560206800, FixedTimespan {  // 1952-03-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-539121600, FixedTimespan {  // 1952-11-01T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-531349200, FixedTimespan {  // 1953-02-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-191361600, FixedTimespan {  // 1963-11-09T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-184194000, FixedTimespan {  // 1964-02-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-155160000, FixedTimespan {  // 1965-00-31T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-150066000, FixedTimespan {  // 1965-02-31T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-128894400, FixedTimespan {  // 1965-11-01T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-121122000, FixedTimespan {  // 1966-02-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-99950400, FixedTimespan {  // 1966-10-01T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-89586000, FixedTimespan {  // 1967-02-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (-68414400, FixedTimespan {  // 1967-10-01T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (-57963600, FixedTimespan {  // 1968-02-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (499752000, FixedTimespan {  // 1985-10-02T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (511239600, FixedTimespan {  // 1986-02-15T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (530596800, FixedTimespan {  // 1986-09-25T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (540270000, FixedTimespan {  // 1987-01-14T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
        (562132800, FixedTimespan {  // 1987-09-25T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "AMST",
        }),
        (571201200, FixedTimespan {  // 1988-01-07T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AMT",
        }),
    ]},
};


