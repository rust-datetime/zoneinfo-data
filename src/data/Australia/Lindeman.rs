
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Australia/Lindeman",
    timespans: &[
        Timespan {
            offset: 35756,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2366755200),
        },
        Timespan {
            offset: 36000,
            format: "AE%sT",
            saving: Saving::Multiple(&rulesets::Aus),
            start_time: Some(-2366755200),
            end_time:   Some(31536000),
        },
        Timespan {
            offset: 36000,
            format: "AE%sT",
            saving: Saving::Multiple(&rulesets::AQ),
            start_time: Some(31536000),
            end_time:   Some(709948800),
        },
        Timespan {
            offset: 36000,
            format: "AE%sT",
            saving: Saving::Multiple(&rulesets::Holiday),
            start_time: Some(709948800),
            end_time:   None,
        },
    ],
};

