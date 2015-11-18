
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Taipei",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 29160,  // UTC offset 29160, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2335248360, FixedTimespan {  // 1895-11-31T15-54-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("JWST"),
        }),
        (-1017820800, FixedTimespan {  // 1937-08-30T16-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("JST"),
        }),
        (-766224000, FixedTimespan {  // 1945-08-20T16-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-745833600, FixedTimespan {  // 1946-04-14T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-733827600, FixedTimespan {  // 1946-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-716889600, FixedTimespan {  // 1947-03-14T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-699613200, FixedTimespan {  // 1947-09-31T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-683884800, FixedTimespan {  // 1948-03-30T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-670669200, FixedTimespan {  // 1948-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-652348800, FixedTimespan {  // 1949-03-30T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-639133200, FixedTimespan {  // 1949-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-620812800, FixedTimespan {  // 1950-03-30T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-607597200, FixedTimespan {  // 1950-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-589276800, FixedTimespan {  // 1951-03-30T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-576061200, FixedTimespan {  // 1951-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-562838400, FixedTimespan {  // 1952-02-01T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-541760400, FixedTimespan {  // 1952-09-31T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-528710400, FixedTimespan {  // 1953-02-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-510224400, FixedTimespan {  // 1953-09-31T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-497174400, FixedTimespan {  // 1954-02-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-478688400, FixedTimespan {  // 1954-09-31T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-465638400, FixedTimespan {  // 1955-02-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-449830800, FixedTimespan {  // 1955-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-434016000, FixedTimespan {  // 1956-02-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-418208400, FixedTimespan {  // 1956-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-402480000, FixedTimespan {  // 1957-02-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-386672400, FixedTimespan {  // 1957-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-370944000, FixedTimespan {  // 1958-02-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-355136400, FixedTimespan {  // 1958-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-339408000, FixedTimespan {  // 1959-02-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-323600400, FixedTimespan {  // 1959-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-302515200, FixedTimespan {  // 1960-04-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-291978000, FixedTimespan {  // 1960-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-270979200, FixedTimespan {  // 1961-04-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-260442000, FixedTimespan {  // 1961-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (133977600, FixedTimespan {  // 1974-02-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (149785200, FixedTimespan {  // 1974-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (165513600, FixedTimespan {  // 1975-02-31T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (181321200, FixedTimespan {  // 1975-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (299606400, FixedTimespan {  // 1979-05-30T16-00-00 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (307551600, FixedTimespan {  // 1979-08-30T15-00-00 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
    ]},
};


