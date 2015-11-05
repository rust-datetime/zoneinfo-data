
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Guyana",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: -7640,  // UTC offset -7640, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-1730584360, ZoneDetails {
            offset: -8100,  // UTC offset -8100, DST offset 0
            name: "GBGT",
        }),
        (-113694300, ZoneDetails {
            offset: -8100,  // UTC offset -8100, DST offset 0
            name: "GYT",
        }),
        (176004900, ZoneDetails {
            offset: -10800,  // UTC offset -10800, DST offset 0
            name: "GYT",
        }),
        (662698800, ZoneDetails {
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "GYT",
        }),
    ]},
};


