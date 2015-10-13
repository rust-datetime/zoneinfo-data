
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Danmarkshavn",
    timespans: &[
        Timespan {
            offset: -2720,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1686096000),
        },
        Timespan {
            offset: -10800,
            format: "WGT",
            saving: Saving::NoSaving,
            start_time: Some(-1686096000),
            end_time:   Some(323834400),
        },
        Timespan {
            offset: -10800,
            format: "WG%sT",
            saving: Saving::Multiple(&rulesets::EU),
            start_time: Some(323834400),
            end_time:   Some(820454400),
        },
        Timespan {
            offset: 0,
            format: "GMT",
            saving: Saving::NoSaving,
            start_time: Some(820454400),
            end_time:   None,
        },
    ],
};

