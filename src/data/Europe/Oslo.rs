
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Oslo",
    timespans: &[
        Timespan {
            offset: 2580,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2366755200),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Norway),
            start_time: Some(-2366755200),
            end_time:   Some(-927507600),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: Some(-927507600),
            end_time:   Some(-781048800),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Norway),
            start_time: Some(-781048800),
            end_time:   Some(315532800),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::EU),
            start_time: Some(315532800),
            end_time:   None,
        },
    ],
};

