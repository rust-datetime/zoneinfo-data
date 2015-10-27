
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Kosrae",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 39116, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2177491916), utc_offset: 39600, dst_offset: 0, name: "KOST" },
        Transition { occurs_at: Some(-7988400), utc_offset: 43200, dst_offset: 0, name: "KOST" },
        Transition { occurs_at: Some(915105600), utc_offset: 39600, dst_offset: 0, name: "KOST" },
    ],
};


