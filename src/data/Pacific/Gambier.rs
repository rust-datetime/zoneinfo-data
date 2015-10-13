
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Gambier",
    timespans: &[
        Timespan {
            offset: -25212,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1806710400),
        },
        Timespan {
            offset: -32400,
            format: "GAMT",
            saving: Saving::NoSaving,
            start_time: Some(-1806710400),
            end_time:   None,
        },
    ],
};

