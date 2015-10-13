
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Rangoon",
    timespans: &[
        Timespan {
            offset: 23080,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2840140800),
        },
        Timespan {
            offset: 23080,
            format: "RMT",
            saving: Saving::NoSaving,
            start_time: Some(-2840140800),
            end_time:   Some(-1577923200),
        },
        Timespan {
            offset: 23400,
            format: "BURT",
            saving: Saving::NoSaving,
            start_time: Some(-1577923200),
            end_time:   Some(-873244800),
        },
        Timespan {
            offset: 32400,
            format: "JST",
            saving: Saving::NoSaving,
            start_time: Some(-873244800),
            end_time:   Some(-778377600),
        },
        Timespan {
            offset: 23400,
            format: "MMT",
            saving: Saving::NoSaving,
            start_time: Some(-778377600),
            end_time:   None,
        },
    ],
};

