
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Indian/Chagos",
    timespans: &[
        Timespan {
            offset: 17380,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1988150400),
        },
        Timespan {
            offset: 18000,
            format: "IOT",
            saving: Saving::NoSaving,
            start_time: Some(-1988150400),
            end_time:   Some(820454400),
        },
        Timespan {
            offset: 21600,
            format: "IOT",
            saving: Saving::NoSaving,
            start_time: Some(820454400),
            end_time:   None,
        },
    ],
};


