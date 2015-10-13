
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Khartoum",
    timespans: &[
        Timespan {
            offset: 7808,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1230768000),
        },
        Timespan {
            offset: 7200,
            format: "CA%sT",
            saving: Saving::Multiple(&rulesets::Sudan),
            start_time: Some(-1230768000),
            end_time:   Some(947937600),
        },
        Timespan {
            offset: 10800,
            format: "EAT",
            saving: Saving::NoSaving,
            start_time: Some(947937600),
            end_time:   None,
        },
    ],
};

