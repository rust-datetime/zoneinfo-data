
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Indian/Christmas",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: 25372,  // UTC offset 25372, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-2364102172, ZoneDetails {
            offset: 25200,  // UTC offset 25200, DST offset 0
            name: "CXT",
        }),
    ]},
};


