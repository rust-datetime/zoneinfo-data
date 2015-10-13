
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Gibraltar",
    timespans: &[
        Timespan {
            offset: 1284,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2821651200),
        },
        Timespan {
            offset: 0,
            format: "%s",
            saving: Saving::Multiple(&rulesets::GB_Eire),
            start_time: Some(-2821651200),
            end_time:   Some(-401320800),
        },
        Timespan {
            offset: 3600,
            format: "CET",
            saving: Saving::NoSaving,
            start_time: Some(-401320800),
            end_time:   Some(378691200),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::EU),
            start_time: Some(378691200),
            end_time:   None,
        },
    ],
};

