
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Brussels",
    timespans: &[
        Timespan {
            offset: 1050,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2840140800),
        },
        Timespan {
            offset: 1050,
            format: "BMT",
            saving: Saving::NoSaving,
            start_time: Some(-2840140800),
            end_time:   Some(-2450952000),
        },
        Timespan {
            offset: 0,
            format: "WET",
            saving: Saving::NoSaving,
            start_time: Some(-2450952000),
            end_time:   Some(-1740355200),
        },
        Timespan {
            offset: 3600,
            format: "CET",
            saving: Saving::NoSaving,
            start_time: Some(-1740355200),
            end_time:   Some(-1693699200),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: Some(-1693699200),
            end_time:   Some(-1613826000),
        },
        Timespan {
            offset: 0,
            format: "WE%sT",
            saving: Saving::Multiple(&rulesets::Belgium),
            start_time: Some(-1613826000),
            end_time:   Some(-934668000),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: Some(-934668000),
            end_time:   Some(-799286400),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Belgium),
            start_time: Some(-799286400),
            end_time:   Some(220924800),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::EU),
            start_time: Some(220924800),
            end_time:   None,
        },
    ],
};

