
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "MST",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   "MST",
        },
        rest: &[
    ]},
};


