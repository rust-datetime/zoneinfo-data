
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Accra",
    timespans: &[
        Timespan {
            offset: 52,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1640995200),
        },
        Timespan {
            offset: 0,
            format: "%s",
            saving: Saving::Multiple(&rulesets::Ghana),
            start_time: Some(-1640995200),
            end_time:   None,
        },
    ],
};


