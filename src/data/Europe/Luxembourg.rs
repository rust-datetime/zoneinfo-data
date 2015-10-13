
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Luxembourg",
    timespans: &[
        Timespan {
            offset: 1476,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2069712000),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Lux),
            start_time: Some(-2069712000),
            end_time:   Some(-1612656000),
        },
        Timespan {
            offset: 0,
            format: "WE%sT",
            saving: Saving::Multiple(&rulesets::Lux),
            start_time: Some(-1612656000),
            end_time:   Some(-1269813600),
        },
        Timespan {
            offset: 0,
            format: "WE%sT",
            saving: Saving::Multiple(&rulesets::Belgium),
            start_time: Some(-1269813600),
            end_time:   Some(-935182800),
        },
        Timespan {
            offset: 3600,
            format: "WE%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: Some(-935182800),
            end_time:   Some(-797979600),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Belgium),
            start_time: Some(-797979600),
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

