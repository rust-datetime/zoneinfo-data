
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Belgrade",
    timespans: &[
        Timespan {
            offset: 4920,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2713910400),
        },
        Timespan {
            offset: 3600,
            format: "CET",
            saving: Saving::NoSaving,
            start_time: Some(-2713910400),
            end_time:   Some(-905821200),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: Some(-905821200),
            end_time:   Some(-788918400),
        },
        Timespan {
            offset: 3600,
            format: "CET",
            saving: Saving::NoSaving,
            start_time: Some(-788918400),
            end_time:   Some(-777938400),
        },
        Timespan {
            offset: 3600,
            format: "CEST",
            saving: Saving::OneOff(3600),
            start_time: Some(-777938400),
            end_time:   Some(-766620000),
        },
        Timespan {
            offset: 3600,
            format: "CET",
            saving: Saving::NoSaving,
            start_time: Some(-766620000),
            end_time:   Some(407203200),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::EU),
            start_time: Some(407203200),
            end_time:   None,
        },
    ],
};


