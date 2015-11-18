
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Phoenix",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -23502,  // UTC offset -23502, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2717646996, FixedTimespan {  // 1883-10-18T18-03-24 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-1633273200, FixedTimespan {  // 1918-02-31T9-00-00 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-1615132800, FixedTimespan {  // 1918-09-27T8-00-00 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-1601823600, FixedTimespan {  // 1919-02-30T9-00-00 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-1583683200, FixedTimespan {  // 1919-09-26T8-00-00 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-880210800, FixedTimespan {  // 1942-01-09T9-00-00 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MWT"),
        }),
        (-820519140, FixedTimespan {  // 1944-00-01T6-01-00 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-812653140, FixedTimespan {  // 1944-03-01T7-01-00 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MWT"),
        }),
        (-796845540, FixedTimespan {  // 1944-09-01T6-01-00 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-84380400, FixedTimespan {  // 1967-03-30T9-00-00 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-68659200, FixedTimespan {  // 1967-09-29T8-00-00 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
    ]},
};


