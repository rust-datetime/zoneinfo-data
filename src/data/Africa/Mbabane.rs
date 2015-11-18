
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Africa/Mbabane",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 6720,  // UTC offset 6720, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2458173120, FixedTimespan {  // 1892-01-07T22-08-00 UTC
            offset: 5400,  // UTC offset 5400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SAST"),
        }),
        (-2109288600, FixedTimespan {  // 1903-01-28T22-30-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SAST"),
        }),
        (-860976000, FixedTimespan {  // 1942-08-20T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SAST"),
        }),
        (-845254800, FixedTimespan {  // 1943-02-20T23-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SAST"),
        }),
        (-829526400, FixedTimespan {  // 1943-08-19T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SAST"),
        }),
        (-813805200, FixedTimespan {  // 1944-02-18T23-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SAST"),
        }),
    ]},
};

