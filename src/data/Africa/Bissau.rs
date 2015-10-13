
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Bissau",
    timespans: &[
        Timespan {
            offset: -3460,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1830384000),
        },
        Timespan {
            offset: -3600,
            format: "WAT",
            saving: Saving::NoSaving,
            start_time: Some(-1830384000),
            end_time:   Some(157766400),
        },
        Timespan {
            offset: 0,
            format: "GMT",
            saving: Saving::NoSaving,
            start_time: Some(157766400),
            end_time:   None,
        },
    ],
};

