
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Port-au-Prince",
    timespans: &[
        Timespan {
            offset: -11440,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2524521600),
        },
        Timespan {
            offset: -11460,
            format: "PPMT",
            saving: Saving::NoSaving,
            start_time: Some(-2524521600),
            end_time:   Some(-1670500800),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::Haiti),
            start_time: Some(-1670500800),
            end_time:   None,
        },
    ],
};

