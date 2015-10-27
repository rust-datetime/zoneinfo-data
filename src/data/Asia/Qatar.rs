
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Qatar",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 12368, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1577935568), utc_offset: 14400, dst_offset: 0, name: "GST" },
        Transition { occurs_at: Some(76190400), utc_offset: 10800, dst_offset: 0, name: "AST" },
    ],
};


