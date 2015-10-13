
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Manila",
    timespans: &[
        Timespan {
            offset: -50640,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-3944678400),
        },
        Timespan {
            offset: 29040,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: Some(-3944678400),
            end_time:   Some(-2229292800),
        },
        Timespan {
            offset: 28800,
            format: "PH%sT",
            saving: Saving::Multiple(&rulesets::Phil),
            start_time: Some(-2229292800),
            end_time:   Some(-873244800),
        },
        Timespan {
            offset: 32400,
            format: "JST",
            saving: Saving::NoSaving,
            start_time: Some(-873244800),
            end_time:   Some(-794188800),
        },
        Timespan {
            offset: 28800,
            format: "PH%sT",
            saving: Saving::Multiple(&rulesets::Phil),
            start_time: Some(-794188800),
            end_time:   None,
        },
    ],
};

