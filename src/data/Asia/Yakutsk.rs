
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Yakutsk",
    timespans: &[
        Timespan {
            offset: 31138,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1579392000),
        },
        Timespan {
            offset: 28800,
            format: "YAKT",
            saving: Saving::NoSaving,
            start_time: Some(-1579392000),
            end_time:   Some(-1247529600),
        },
        Timespan {
            offset: 32400,
            format: "YAK%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(-1247529600),
            end_time:   Some(670384800),
        },
        Timespan {
            offset: 28800,
            format: "YAK%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(670384800),
            end_time:   Some(695786400),
        },
        Timespan {
            offset: 32400,
            format: "YAK%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(695786400),
            end_time:   Some(1301191200),
        },
        Timespan {
            offset: 36000,
            format: "YAKT",
            saving: Saving::NoSaving,
            start_time: Some(1301191200),
            end_time:   Some(1414288800),
        },
        Timespan {
            offset: 32400,
            format: "YAKT",
            saving: Saving::NoSaving,
            start_time: Some(1414288800),
            end_time:   None,
        },
    ],
};

