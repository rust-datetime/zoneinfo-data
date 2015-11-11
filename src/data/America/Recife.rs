
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "America/Recife",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -6024,  // UTC offset -6024, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-1767219576, FixedTimespan {  // 1914-00-01T1-40-24 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-1206957600, FixedTimespan {  // 1931-09-03T14-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-1191362400, FixedTimespan {  // 1932-03-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-1175374800, FixedTimespan {  // 1932-09-03T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-1159826400, FixedTimespan {  // 1933-03-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-633819600, FixedTimespan {  // 1949-11-01T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-622069200, FixedTimespan {  // 1950-03-16T3-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-602283600, FixedTimespan {  // 1950-11-01T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-591832800, FixedTimespan {  // 1951-03-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-570747600, FixedTimespan {  // 1951-11-01T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-560210400, FixedTimespan {  // 1952-03-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-539125200, FixedTimespan {  // 1952-11-01T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-531352800, FixedTimespan {  // 1953-02-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-191365200, FixedTimespan {  // 1963-11-09T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-184197600, FixedTimespan {  // 1964-02-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-155163600, FixedTimespan {  // 1965-00-31T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-150069600, FixedTimespan {  // 1965-02-31T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-128898000, FixedTimespan {  // 1965-11-01T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-121125600, FixedTimespan {  // 1966-02-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-99954000, FixedTimespan {  // 1966-10-01T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-89589600, FixedTimespan {  // 1967-02-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (-68418000, FixedTimespan {  // 1967-10-01T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (-57967200, FixedTimespan {  // 1968-02-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (499748400, FixedTimespan {  // 1985-10-02T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (511236000, FixedTimespan {  // 1986-02-15T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (530593200, FixedTimespan {  // 1986-09-25T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (540266400, FixedTimespan {  // 1987-01-14T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (562129200, FixedTimespan {  // 1987-09-25T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (571197600, FixedTimespan {  // 1988-01-07T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (592974000, FixedTimespan {  // 1988-09-16T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (602042400, FixedTimespan {  // 1989-00-29T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (624423600, FixedTimespan {  // 1989-09-15T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (634701600, FixedTimespan {  // 1990-01-11T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (938919600, FixedTimespan {  // 1999-09-03T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (951616800, FixedTimespan {  // 2000-01-27T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (970974000, FixedTimespan {  // 2000-09-08T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (971575200, FixedTimespan {  // 2000-09-15T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
        (1003028400, FixedTimespan {  // 2001-09-14T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "BRST",
        }),
        (1013911200, FixedTimespan {  // 2002-01-17T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "BRT",
        }),
    ]},
};


