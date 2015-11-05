
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Nairobi",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: 8836,  // UTC offset 8836, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-1309746436, ZoneDetails {
            offset: 10800,  // UTC offset 10800, DST offset 0
            name: "EAT",
        }),
        (-1262314800, ZoneDetails {
            offset: 9000,  // UTC offset 9000, DST offset 0
            name: "BEAT",
        }),
        (-946780200, ZoneDetails {
            offset: 9900,  // UTC offset 9900, DST offset 0
            name: "BEAUT",
        }),
        (-315629100, ZoneDetails {
            offset: 10800,  // UTC offset 10800, DST offset 0
            name: "EAT",
        }),
    ]},
};


