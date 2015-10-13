
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Novosibirsk",
    timespans: &[
        Timespan {
            offset: 19900,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1579456800),
        },
        Timespan {
            offset: 21600,
            format: "NOVT",
            saving: Saving::NoSaving,
            start_time: Some(-1579456800),
            end_time:   Some(-1247529600),
        },
        Timespan {
            offset: 25200,
            format: "NOV%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(-1247529600),
            end_time:   Some(670384800),
        },
        Timespan {
            offset: 21600,
            format: "NOV%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(670384800),
            end_time:   Some(695786400),
        },
        Timespan {
            offset: 25200,
            format: "NOV%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(695786400),
            end_time:   Some(738115200),
        },
        Timespan {
            offset: 21600,
            format: "NOV%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(738115200),
            end_time:   Some(1301191200),
        },
        Timespan {
            offset: 25200,
            format: "NOVT",
            saving: Saving::NoSaving,
            start_time: Some(1301191200),
            end_time:   Some(1414288800),
        },
        Timespan {
            offset: 21600,
            format: "NOVT",
            saving: Saving::NoSaving,
            start_time: Some(1414288800),
            end_time:   None,
        },
    ],
};

