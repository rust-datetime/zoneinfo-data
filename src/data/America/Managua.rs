
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Managua",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -15292,  // UTC offset -15292, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2524506308, FixedTimespan {  // 1890-01-01T04:14:52.000 UTC
            offset: -15288,  // UTC offset -15288, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MMT"),
        }),
        (-1121111112, FixedTimespan {  // 1934-06-23T04:14:48.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (105084000, FixedTimespan {  // 1973-05-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (161758800, FixedTimespan {  // 1975-02-16T05:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (290584800, FixedTimespan {  // 1979-03-18T06:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (299134800, FixedTimespan {  // 1979-06-25T05:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (322034400, FixedTimespan {  // 1980-03-16T06:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (330584400, FixedTimespan {  // 1980-06-23T05:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (694260000, FixedTimespan {  // 1992-01-01T10:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (717310800, FixedTimespan {  // 1992-09-24T05:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (725868000, FixedTimespan {  // 1993-01-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (852094800, FixedTimespan {  // 1997-01-01T05:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1113112800, FixedTimespan {  // 2005-04-10T06:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1128229200, FixedTimespan {  // 2005-10-02T05:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1146384000, FixedTimespan {  // 2006-04-30T08:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1159682400, FixedTimespan {  // 2006-10-01T06:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
    ]},
};


