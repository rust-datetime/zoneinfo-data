
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Pago_Pago",
    transitions: ZoneSet {
        first: ZoneDetails {
            offset: 45432,  // UTC offset 45432, DST offset 0
            name: "LMT",
        },
        rest: &[
        (-2855738232, ZoneDetails {
            offset: -38232,  // UTC offset -38232, DST offset 0
            name: "LMT",
        }),
        (-1861881768, ZoneDetails {
            offset: -39600,  // UTC offset -39600, DST offset 0
            name: "NST",
        }),
        (-86878800, ZoneDetails {
            offset: -39600,  // UTC offset -39600, DST offset 0
            name: "BST",
        }),
        (439038000, ZoneDetails {
            offset: -39600,  // UTC offset -39600, DST offset 0
            name: "SST",
        }),
    ]},
};


