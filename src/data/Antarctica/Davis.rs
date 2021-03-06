
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Antarctica/Davis",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("zzz"),
        },
        rest: &[
        (-409190400, FixedTimespan {  // 1957-01-13T00:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("DAVT"),
        }),
        (-163062000, FixedTimespan {  // 1964-10-31T17:00:00.000 UTC
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("zzz"),
        }),
        (-28857600, FixedTimespan {  // 1969-02-01T00:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("DAVT"),
        }),
        (1255806000, FixedTimespan {  // 2009-10-17T19:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("DAVT"),
        }),
        (1268233200, FixedTimespan {  // 2010-03-10T15:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("DAVT"),
        }),
        (1319742000, FixedTimespan {  // 2011-10-27T19:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("DAVT"),
        }),
        (1329836400, FixedTimespan {  // 2012-02-21T15:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("DAVT"),
        }),
    ]},
};


