
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Thunder_Bay",
    timespans: &[
        Timespan {
            offset: -14580,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2366755200),
        },
        Timespan {
            offset: -21600,
            format: "CST",
            saving: Saving::NoSaving,
            start_time: Some(-2366755200),
            end_time:   Some(-1893456000),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(-1893456000),
            end_time:   Some(-883612800),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(-883612800),
            end_time:   Some(0),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::Toronto),
            start_time: Some(0),
            end_time:   Some(94694400),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(94694400),
            end_time:   Some(126230400),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(126230400),
            end_time:   None,
        },
    ],
};

