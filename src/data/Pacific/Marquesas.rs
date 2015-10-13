
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Marquesas",
    timespans: &[
        Timespan {
            offset: -31320,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1806710400),
        },
        Timespan {
            offset: -30600,
            format: "MART",
            saving: Saving::NoSaving,
            start_time: Some(-1806710400),
            end_time:   None,
        },
    ],
};


