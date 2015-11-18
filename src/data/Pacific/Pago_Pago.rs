
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Pago_Pago",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 45432,  // UTC offset 45432, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2855738232, FixedTimespan {  // 1879-06-04T11-22-48 UTC
            offset: -38232,  // UTC offset -38232, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        }),
        (-1861881768, FixedTimespan {  // 1911-00-01T10-37-12 UTC
            offset: -39600,  // UTC offset -39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NST"),
        }),
        (-86878800, FixedTimespan {  // 1967-03-01T11-00-00 UTC
            offset: -39600,  // UTC offset -39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BST"),
        }),
        (439038000, FixedTimespan {  // 1983-10-30T11-00-00 UTC
            offset: -39600,  // UTC offset -39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SST"),
        }),
    ]},
};


