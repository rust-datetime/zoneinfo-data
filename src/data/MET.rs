
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "MET",
    timespans: &[
        Timespan {
            offset: 3600,
            format: "ME%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: None,
            end_time:   None,
        },
    ],
};

