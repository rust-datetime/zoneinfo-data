
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Asia/Pyongyang",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 30180,  // UTC offset 30180, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-1948782180, FixedTimespan {  // 1908-02-31T15-37-00 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   "KST",
        }),
        (-1830414600, FixedTimespan {  // 1911-11-31T15-30-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JCST",
        }),
        (-1017824400, FixedTimespan {  // 1937-08-30T15-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "JST",
        }),
        (-768646800, FixedTimespan {  // 1945-07-23T15-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   "KST",
        }),
        (1439564400, FixedTimespan {  // 2015-07-14T15-00-00 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   "KST",
        }),
    ]},
};


