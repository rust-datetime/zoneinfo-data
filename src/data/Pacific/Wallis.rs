
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Wallis",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 44120,  // UTC offset 44120, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2177496920, FixedTimespan {  // 1900-11-31T11-44-40 UTC
            offset: 43200,  // UTC offset 43200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WFT"),
        }),
    ]},
};


