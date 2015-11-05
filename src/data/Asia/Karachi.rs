
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Karachi",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: 16092,  // UTC offset 16092, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-1988166492, ZoneDetails {
            offset: 19800,  // UTC offset 19800, DST offset 0
            name: "IST",
        }),
        (-862637400, ZoneDetails {
            offset: 23400,  // UTC offset 19800, DST offset 3600
            name: "IST",
        }),
        (-764145000, ZoneDetails {
            offset: 19800,  // UTC offset 19800, DST offset 0
            name: "IST",
        }),
        (-576135000, ZoneDetails {
            offset: 18000,  // UTC offset 18000, DST offset 0
            name: "KART",
        }),
        (38775600, ZoneDetails {
            offset: 18000,  // UTC offset 18000, DST offset 0
            name: "PKT",
        }),
        (1018119660, ZoneDetails {
            offset: 21600,  // UTC offset 18000, DST offset 3600
            name: "PKST",
        }),
        (1033840860, ZoneDetails {
            offset: 18000,  // UTC offset 18000, DST offset 0
            name: "PKT",
        }),
        (1212260400, ZoneDetails {
            offset: 21600,  // UTC offset 18000, DST offset 3600
            name: "PKST",
        }),
        (1225476000, ZoneDetails {
            offset: 18000,  // UTC offset 18000, DST offset 0
            name: "PKT",
        }),
        (1239735600, ZoneDetails {
            offset: 21600,  // UTC offset 18000, DST offset 3600
            name: "PKST",
        }),
        (1257012000, ZoneDetails {
            offset: 18000,  // UTC offset 18000, DST offset 0
            name: "PKT",
        }),
    ]},
};


