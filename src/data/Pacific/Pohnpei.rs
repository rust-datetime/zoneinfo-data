
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Pohnpei",
    timespans: &[
        Timespan {
            offset: 37972,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2177452800),
        },
        Timespan {
            offset: 39600,
            format: "PONT",
            saving: Saving::NoSaving,
            start_time: Some(-2177452800),
            end_time:   None,
        },
    ],
};

