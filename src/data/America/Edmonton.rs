
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Edmonton",
    timespans: &[
        Timespan {
            offset: -23168,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1998691200),
        },
        Timespan {
            offset: -25200,
            format: "M%sT",
            saving: Saving::Multiple(&rulesets::Edm),
            start_time: Some(-1998691200),
            end_time:   Some(536457600),
        },
        Timespan {
            offset: -25200,
            format: "M%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(536457600),
            end_time:   None,
        },
    ],
};

