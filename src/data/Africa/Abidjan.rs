
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Abidjan",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: 968,  // UTC offset 968, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-1830384968, ZoneDetails {
            offset: 0,  // UTC offset 0, DST offset 0
            name: "GMT",
        }),
    ]},
};


