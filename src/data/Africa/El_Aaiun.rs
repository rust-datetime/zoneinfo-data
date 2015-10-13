
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/El_Aaiun",
    timespans: &[
        Timespan {
            offset: 3168,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1136073600),
        },
        Timespan {
            offset: -3600,
            format: "WAT",
            saving: Saving::NoSaving,
            start_time: Some(-1136073600),
            end_time:   Some(198288000),
        },
        Timespan {
            offset: 0,
            format: "WE%sT",
            saving: Saving::Multiple(&rulesets::Morocco),
            start_time: Some(198288000),
            end_time:   None,
        },
    ],
};

