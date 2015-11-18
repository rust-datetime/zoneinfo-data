
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Karachi",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 16092,  // UTC offset 16092, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1988166492, FixedTimespan {  // 1906-11-31T19-31-48 UTC
            offset: 19800,  // UTC offset 19800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IST"),
        }),
        (-862637400, FixedTimespan {  // 1942-07-31T18-30-00 UTC
            offset: 23400,  // UTC offset 19800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IST"),
        }),
        (-764145000, FixedTimespan {  // 1945-09-14T17-30-00 UTC
            offset: 19800,  // UTC offset 19800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IST"),
        }),
        (-576135000, FixedTimespan {  // 1951-08-29T18-30-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KART"),
        }),
        (38775600, FixedTimespan {  // 1971-02-25T19-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PKT"),
        }),
        (1018119660, FixedTimespan {  // 2002-03-06T19-01-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PKST"),
        }),
        (1033840860, FixedTimespan {  // 2002-09-05T18-01-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PKT"),
        }),
        (1212260400, FixedTimespan {  // 2008-04-31T19-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PKST"),
        }),
        (1225476000, FixedTimespan {  // 2008-09-31T18-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PKT"),
        }),
        (1239735600, FixedTimespan {  // 2009-03-14T19-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PKST"),
        }),
        (1257012000, FixedTimespan {  // 2009-09-31T18-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PKT"),
        }),
    ]},
};


