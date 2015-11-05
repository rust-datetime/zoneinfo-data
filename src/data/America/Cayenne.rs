
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Cayenne",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: -9040,  // UTC offset -9040, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-1846272560, ZoneDetails {
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "GFT",
        }),
        (-71092800, ZoneDetails {
            offset: -10800,  // UTC offset -10800, DST offset 0
            name: "GFT",
        }),
    ]},
};


