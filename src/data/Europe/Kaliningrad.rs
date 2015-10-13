
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Kaliningrad",
    timespans: &[
        Timespan {
            offset: 4920,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2422051200),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: Some(-2422051200),
            end_time:   Some(-788918400),
        },
        Timespan {
            offset: 7200,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Poland),
            start_time: Some(-788918400),
            end_time:   Some(-757382400),
        },
        Timespan {
            offset: 10800,
            format: "MSK/MSD",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(-757382400),
            end_time:   Some(670384800),
        },
        Timespan {
            offset: 7200,
            format: "EE%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(670384800),
            end_time:   Some(1301191200),
        },
        Timespan {
            offset: 10800,
            format: "FET",
            saving: Saving::NoSaving,
            start_time: Some(1301191200),
            end_time:   Some(1414288800),
        },
        Timespan {
            offset: 7200,
            format: "EET",
            saving: Saving::NoSaving,
            start_time: Some(1414288800),
            end_time:   None,
        },
    ],
};

