
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Wake",
    timespans: &[
        Timespan {
            offset: 39988,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2177452800),
        },
        Timespan {
            offset: 43200,
            format: "WAKT",
            saving: Saving::NoSaving,
            start_time: Some(-2177452800),
            end_time:   None,
        },
    ],
};


