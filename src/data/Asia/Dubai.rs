
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Dubai",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 13272, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1577936472), utc_offset: 14400, dst_offset: 0, name: "GST" },
    ],
};


