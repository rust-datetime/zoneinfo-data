
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Creston",
    timespans: &[
        Timespan {
            offset: -22436,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2713910400),
        },
        Timespan {
            offset: -25200,
            format: "MST",
            saving: Saving::NoSaving,
            start_time: Some(-2713910400),
            end_time:   Some(-1680480000),
        },
        Timespan {
            offset: -28800,
            format: "PST",
            saving: Saving::NoSaving,
            start_time: Some(-1680480000),
            end_time:   Some(-1627862400),
        },
        Timespan {
            offset: -25200,
            format: "MST",
            saving: Saving::NoSaving,
            start_time: Some(-1627862400),
            end_time:   None,
        },
    ],
};

