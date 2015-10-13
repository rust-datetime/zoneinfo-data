
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Moncton",
    timespans: &[
        Timespan {
            offset: -13252,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2715897600),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(-2715897600),
            end_time:   Some(-2131660800),
        },
        Timespan {
            offset: -14400,
            format: "A%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(-2131660800),
            end_time:   Some(-1167609600),
        },
        Timespan {
            offset: -14400,
            format: "A%sT",
            saving: Saving::Multiple(&rulesets::Moncton),
            start_time: Some(-1167609600),
            end_time:   Some(-883612800),
        },
        Timespan {
            offset: -14400,
            format: "A%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(-883612800),
            end_time:   Some(-757382400),
        },
        Timespan {
            offset: -14400,
            format: "A%sT",
            saving: Saving::Multiple(&rulesets::Moncton),
            start_time: Some(-757382400),
            end_time:   Some(94694400),
        },
        Timespan {
            offset: -14400,
            format: "A%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(94694400),
            end_time:   Some(725846400),
        },
        Timespan {
            offset: -14400,
            format: "A%sT",
            saving: Saving::Multiple(&rulesets::Moncton),
            start_time: Some(725846400),
            end_time:   Some(1167609600),
        },
        Timespan {
            offset: -14400,
            format: "A%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(1167609600),
            end_time:   None,
        },
    ],
};


