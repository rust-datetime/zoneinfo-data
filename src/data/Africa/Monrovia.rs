
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Africa/Monrovia",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 2588,  // UTC offset 2588, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2776984988, FixedTimespan {  // 1881-11-31T23-16-52 UTC
            offset: 2588,  // UTC offset 2588, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MMT"),
        }),
        (-1604364188, FixedTimespan {  // 1919-01-28T23-16-52 UTC
            offset: 2670,  // UTC offset 2670, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LRT"),
        }),
        (73523730, FixedTimespan {  // 1972-03-30T23-15-30 UTC
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GMT"),
        }),
    ]},
};


