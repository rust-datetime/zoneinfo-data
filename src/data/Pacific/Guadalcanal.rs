
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Guadalcanal",
    timespans: &[
        Timespan {
            offset: 38388,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1806710400),
        },
        Timespan {
            offset: 39600,
            format: "SBT",
            saving: Saving::NoSaving,
            start_time: Some(-1806710400),
            end_time:   None,
        },
    ],
};

