
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "America/Managua",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -15292,  // UTC offset -15292, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2524506308, FixedTimespan {  // 1890-00-01T4-14-52 UTC
            offset: -15288,  // UTC offset -15288, DST offset 0
            is_dst: false,
            name:   "MMT",
        }),
        (-1121111112, FixedTimespan {  // 1934-05-23T4-14-48 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (105084000, FixedTimespan {  // 1973-04-01T6-00-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   "EST",
        }),
        (161758800, FixedTimespan {  // 1975-01-16T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (290584800, FixedTimespan {  // 1979-02-18T6-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   "CDT",
        }),
        (299134800, FixedTimespan {  // 1979-05-25T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (322034400, FixedTimespan {  // 1980-02-16T6-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   "CDT",
        }),
        (330584400, FixedTimespan {  // 1980-05-23T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (694260000, FixedTimespan {  // 1992-00-01T10-00-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   "EST",
        }),
        (717310800, FixedTimespan {  // 1992-08-24T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (725868000, FixedTimespan {  // 1993-00-01T6-00-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   "EST",
        }),
        (852094800, FixedTimespan {  // 1997-00-01T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (1113112800, FixedTimespan {  // 2005-03-10T6-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   "CDT",
        }),
        (1128229200, FixedTimespan {  // 2005-09-02T5-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
        (1146384000, FixedTimespan {  // 2006-03-30T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   "CDT",
        }),
        (1159682400, FixedTimespan {  // 2006-09-01T6-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   "CST",
        }),
    ]},
};


