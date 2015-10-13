
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Budapest",
    timespans: &[
        Timespan {
            offset: 4580,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2500934400),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: Some(-2500934400),
            end_time:   Some(-1640995200),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Hungary),
            start_time: Some(-1640995200),
            end_time:   Some(-906768000),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: Some(-906768000),
            end_time:   Some(-788918400),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Hungary),
            start_time: Some(-788918400),
            end_time:   Some(338954400),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::EU),
            start_time: Some(338954400),
            end_time:   None,
        },
    ],
};

