
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Panama",
    timespans: &[
        Timespan {
            offset: -16912,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2524521600),
        },
        Timespan {
            offset: -16824,
            format: "CMT",
            saving: Saving::NoSaving,
            start_time: Some(-2524521600),
            end_time:   Some(-1946937600),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(-1946937600),
            end_time:   None,
        },
    ],
};

