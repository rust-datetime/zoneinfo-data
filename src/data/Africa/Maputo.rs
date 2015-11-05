
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Maputo",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: 7820,  // UTC offset 7820, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-2109291020, ZoneDetails {
            offset: 7200,  // UTC offset 7200, DST offset 0
            name: "CAT",
        }),
    ]},
};


