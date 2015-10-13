
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Asuncion",
    timespans: &[
        Timespan {
            offset: -7760,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2524521600),
        },
        Timespan {
            offset: -7760,
            format: "AMT",
            saving: Saving::NoSaving,
            start_time: Some(-2524521600),
            end_time:   Some(-1206403200),
        },
        Timespan {
            offset: -14400,
            format: "PYT",
            saving: Saving::NoSaving,
            start_time: Some(-1206403200),
            end_time:   Some(86745600),
        },
        Timespan {
            offset: -10800,
            format: "PYT",
            saving: Saving::NoSaving,
            start_time: Some(86745600),
            end_time:   Some(134006400),
        },
        Timespan {
            offset: -14400,
            format: "PY%sT",
            saving: Saving::Multiple(&rulesets::Para),
            start_time: Some(134006400),
            end_time:   None,
        },
    ],
};

