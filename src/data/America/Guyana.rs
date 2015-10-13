
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Guyana",
    timespans: &[
        Timespan {
            offset: -7640,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1730592000),
        },
        Timespan {
            offset: -8100,
            format: "GBGT",
            saving: Saving::NoSaving,
            start_time: Some(-1730592000),
            end_time:   Some(-113702400),
        },
        Timespan {
            offset: -8100,
            format: "GYT",
            saving: Saving::NoSaving,
            start_time: Some(-113702400),
            end_time:   Some(175996800),
        },
        Timespan {
            offset: -10800,
            format: "GYT",
            saving: Saving::NoSaving,
            start_time: Some(175996800),
            end_time:   Some(662688000),
        },
        Timespan {
            offset: -14400,
            format: "GYT",
            saving: Saving::NoSaving,
            start_time: Some(662688000),
            end_time:   None,
        },
    ],
};

