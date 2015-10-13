
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Indiana/Winamac",
    timespans: &[
        Timespan {
            offset: -15215,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2717667985),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(-2717667985),
            end_time:   Some(-757382400),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::Pulaski),
            start_time: Some(-757382400),
            end_time:   Some(-273708000),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(-273708000),
            end_time:   Some(-31536000),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(-31536000),
            end_time:   Some(31536000),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(31536000),
            end_time:   Some(1143943200),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(1143943200),
            end_time:   Some(1173578400),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(1173578400),
            end_time:   None,
        },
    ],
};


