
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Colombo",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 19164, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2840159964), utc_offset: 19172, dst_offset: 0, name: "MMT" },
        Transition { occurs_at: Some(-2019705572), utc_offset: 19800, dst_offset: 0, name: "IST" },
        Transition { occurs_at: Some(-883287000), utc_offset: 19800, dst_offset: 1800, name: "IHST" },
        Transition { occurs_at: Some(-862639200), utc_offset: 19800, dst_offset: 3600, name: "IST" },
        Transition { occurs_at: Some(-764051400), utc_offset: 19800, dst_offset: 0, name: "IST" },
        Transition { occurs_at: Some(832962600), utc_offset: 23400, dst_offset: 0, name: "LKT" },
        Transition { occurs_at: Some(846266400), utc_offset: 21600, dst_offset: 0, name: "LKT" },
        Transition { occurs_at: Some(1145039400), utc_offset: 19800, dst_offset: 0, name: "IST" },
    ],
};


