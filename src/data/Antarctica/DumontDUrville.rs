
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Antarctica/DumontDUrville",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: 0,  // UTC offset 0, DST offset 0
            name: "zzz",
        },
        rest: &[
        (-725846400, ZoneDetails {
            offset: 36000,  // UTC offset 36000, DST offset 0
            name: "PMT",
        }),
        (-566992800, ZoneDetails {
            offset: 0,  // UTC offset 0, DST offset 0
            name: "zzz",
        }),
        (-415497600, ZoneDetails {
            offset: 36000,  // UTC offset 36000, DST offset 0
            name: "DDUT",
        }),
    ]},
};


