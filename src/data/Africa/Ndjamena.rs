
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Ndjamena",
    timespans: &[
        Timespan {
            offset: 3612,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1830384000),
        },
        Timespan {
            offset: 3600,
            format: "WAT",
            saving: Saving::NoSaving,
            start_time: Some(-1830384000),
            end_time:   Some(308707200),
        },
        Timespan {
            offset: 3600,
            format: "WAST",
            saving: Saving::OneOff(3600),
            start_time: Some(308707200),
            end_time:   Some(321321600),
        },
        Timespan {
            offset: 3600,
            format: "WAT",
            saving: Saving::NoSaving,
            start_time: Some(321321600),
            end_time:   None,
        },
    ],
};

