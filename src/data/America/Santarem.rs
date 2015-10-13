
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Santarem",
    timespans: &[
        Timespan {
            offset: -8472,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1767225600),
        },
        Timespan {
            offset: -14400,
            format: "AM%sT",
            saving: Saving::Multiple(&rulesets::Brazil),
            start_time: Some(-1767225600),
            end_time:   Some(590025600),
        },
        Timespan {
            offset: -14400,
            format: "AMT",
            saving: Saving::NoSaving,
            start_time: Some(590025600),
            end_time:   Some(1214265600),
        },
        Timespan {
            offset: -10800,
            format: "BRT",
            saving: Saving::NoSaving,
            start_time: Some(1214265600),
            end_time:   None,
        },
    ],
};

