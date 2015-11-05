
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/La_Paz",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: -12444,  // UTC offset -12444, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-2524509156, ZoneDetails {
            offset: -12444,  // UTC offset -12444, DST offset 0
            name: "CMT",
        }),
        (-1205958756, ZoneDetails {
            offset: -8844,  // UTC offset -12444, DST offset 3600
            name: "BOST",
        }),
        (-1192311156, ZoneDetails {
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "BOT",
        }),
    ]},
};


