
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Australia/Darwin",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 31400, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2364108200), utc_offset: 32400, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-2230189200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-1672565340), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(-1665390600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-883639800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(-876126600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-860398200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(-844677000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-828343800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(-813227400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
    ],
};


