
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Whitehorse",
    timespans: &[
        Timespan {
            offset: -32388,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2189030400),
        },
        Timespan {
            offset: -32400,
            format: "Y%sT",
            saving: Saving::Multiple(&rulesets::NT_YK),
            start_time: Some(-2189030400),
            end_time:   Some(-81993600),
        },
        Timespan {
            offset: -28800,
            format: "P%sT",
            saving: Saving::Multiple(&rulesets::NT_YK),
            start_time: Some(-81993600),
            end_time:   Some(315532800),
        },
        Timespan {
            offset: -28800,
            format: "P%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(315532800),
            end_time:   None,
        },
    ],
};


