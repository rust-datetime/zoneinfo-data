
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Indiana/Indianapolis",
    timespans: &[
        Timespan {
            offset: -15322,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2717667878),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(-2717667878),
            end_time:   Some(-1577923200),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::Indianapolis),
            start_time: Some(-1577923200),
            end_time:   Some(-883612800),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(-883612800),
            end_time:   Some(-757382400),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::Indianapolis),
            start_time: Some(-757382400),
            end_time:   Some(-463615200),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(-463615200),
            end_time:   Some(-386805600),
        },
        Timespan {
            offset: -21600,
            format: "CST",
            saving: Saving::NoSaving,
            start_time: Some(-386805600),
            end_time:   Some(-368661600),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(-368661600),
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
            end_time:   Some(1136073600),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(1136073600),
            end_time:   None,
        },
    ],
};


