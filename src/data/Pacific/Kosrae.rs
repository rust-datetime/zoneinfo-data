
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Kosrae",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 39116,  // UTC offset 39116, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2177491916, FixedTimespan {  // 1900-11-31T13-08-04 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KOST"),
        }),
        (-7988400, FixedTimespan {  // 1969-08-30T13-00-00 UTC
            offset: 43200,  // UTC offset 43200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KOST"),
        }),
        (915105600, FixedTimespan {  // 1998-11-31T12-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KOST"),
        }),
    ]},
};


