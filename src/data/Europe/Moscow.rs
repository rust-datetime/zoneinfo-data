
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Moscow",
    timespans: &[
        Timespan {
            offset: 9017,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2840140800),
        },
        Timespan {
            offset: 9017,
            format: "MMT",
            saving: Saving::NoSaving,
            start_time: Some(-2840140800),
            end_time:   Some(-1688256000),
        },
        Timespan {
            offset: 9079,
            format: "%s",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(-1688256000),
            end_time:   Some(-1593813600),
        },
        Timespan {
            offset: 10800,
            format: "%s",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(-1593813600),
            end_time:   Some(-1522713600),
        },
        Timespan {
            offset: 10800,
            format: "MSK/MSD",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(-1522713600),
            end_time:   Some(-1491177600),
        },
        Timespan {
            offset: 7200,
            format: "EET",
            saving: Saving::NoSaving,
            start_time: Some(-1491177600),
            end_time:   Some(-1247529600),
        },
        Timespan {
            offset: 10800,
            format: "MSK/MSD",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(-1247529600),
            end_time:   Some(670384800),
        },
        Timespan {
            offset: 7200,
            format: "EE%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(670384800),
            end_time:   Some(695786400),
        },
        Timespan {
            offset: 10800,
            format: "MSK/MSD",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(695786400),
            end_time:   Some(1301191200),
        },
        Timespan {
            offset: 14400,
            format: "MSK",
            saving: Saving::NoSaving,
            start_time: Some(1301191200),
            end_time:   Some(1414288800),
        },
        Timespan {
            offset: 10800,
            format: "MSK",
            saving: Saving::NoSaving,
            start_time: Some(1414288800),
            end_time:   None,
        },
    ],
};

