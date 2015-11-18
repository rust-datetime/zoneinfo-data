
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Atikokan",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -21212,  // UTC offset -21212, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2366733988, FixedTimespan {  // 1895-00-01T5-53-32 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-1632067200, FixedTimespan {  // 1918-03-14T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-1615136400, FixedTimespan {  // 1918-09-27T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-923248800, FixedTimespan {  // 1940-08-29T6-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-880218000, FixedTimespan {  // 1942-01-09T7-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CWT"),
        }),
        (-769395600, FixedTimespan {  // 1945-07-14T23-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CPT"),
        }),
        (-765392400, FixedTimespan {  // 1945-08-30T7-00-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
    ]},
};


