
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Atlantic/Faroe",
    timespans: &[
        Timespan {
            offset: 1624,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1955750400),
        },
        Timespan {
            offset: 0,
            format: "WET",
            saving: Saving::NoSaving,
            start_time: Some(-1955750400),
            end_time:   Some(347155200),
        },
        Timespan {
            offset: 0,
            format: "WE%sT",
            saving: Saving::Multiple(&rulesets::EU),
            start_time: Some(347155200),
            end_time:   None,
        },
    ],
};


