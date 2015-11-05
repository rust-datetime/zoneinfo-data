
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Marquesas",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: -31320,  // UTC offset -31320, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-1806679080, ZoneDetails {
            offset: -30600,  // UTC offset -30600, DST offset 0
            name: "MART",
        }),
    ]},
};


