
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Detroit",
    timespans: &[
        Timespan {
            offset: -16069,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2051222400),
        },
        Timespan {
            offset: -21600,
            format: "CST",
            saving: Saving::NoSaving,
            start_time: Some(-2051222400),
            end_time:   Some(-1724104800),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(-1724104800),
            end_time:   Some(-883612800),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(-883612800),
            end_time:   Some(-757382400),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::Detroit),
            start_time: Some(-757382400),
            end_time:   Some(94694400),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(94694400),
            end_time:   Some(157766400),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(157766400),
            end_time:   Some(167796000),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(167796000),
            end_time:   None,
        },
    ],
};

