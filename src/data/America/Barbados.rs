
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Barbados",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -7291,  // UTC offset -7291, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1451685509, FixedTimespan {  // 1924-00-01T2-01-31 UTC
            offset: -7291,  // UTC offset -7291, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BMT"),
        }),
        (-1199224709, FixedTimespan {  // 1932-00-01T2-01-31 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (234943200, FixedTimespan {  // 1977-05-12T6-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (244616400, FixedTimespan {  // 1977-09-02T5-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (261554400, FixedTimespan {  // 1978-03-16T6-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (276066000, FixedTimespan {  // 1978-09-01T5-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (293004000, FixedTimespan {  // 1979-03-15T6-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (307515600, FixedTimespan {  // 1979-08-30T5-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (325058400, FixedTimespan {  // 1980-03-20T6-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (338706000, FixedTimespan {  // 1980-08-25T5-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
    ]},
};


