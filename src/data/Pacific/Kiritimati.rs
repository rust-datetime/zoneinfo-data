
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Kiritimati",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: -34240,  // UTC offset -34240, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-2177418560, ZoneDetails {
            offset: -33600,  // UTC offset -33600, DST offset 0
            name: "LINT",
        }),
        (307617600, ZoneDetails {
            offset: -36000,  // UTC offset -36000, DST offset 0
            name: "LINT",
        }),
        (788954400, ZoneDetails {
            offset: 50400,  // UTC offset 50400, DST offset 0
            name: "LINT",
        }),
    ]},
};


