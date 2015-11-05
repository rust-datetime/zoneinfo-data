
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Antarctica/Mawson",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: 0,  // UTC offset 0, DST offset 0
            name: "zzz",
        },
        rest: &[
        (-501206400, ZoneDetails {
            offset: 21600,  // UTC offset 21600, DST offset 0
            name: "MAWT",
        }),
        (1255809600, ZoneDetails {
            offset: 18000,  // UTC offset 18000, DST offset 0
            name: "MAWT",
        }),
    ]},
};


