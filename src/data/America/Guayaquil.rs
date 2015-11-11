
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "America/Guayaquil",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -16840,  // UTC offset -16840, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2524504760, FixedTimespan {  // 1890-00-01T4-40-40 UTC
            offset: -17160,  // UTC offset -17160, DST offset 0
            is_dst: false,
            name:   "QMT",
        }),
        (-1230750840, FixedTimespan {  // 1931-00-01T4-46-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   "ECT",
        }),
    ]},
};


