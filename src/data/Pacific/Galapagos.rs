
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Galapagos",
    timespans: &[
        Timespan {
            offset: -14496,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1230768000),
        },
        Timespan {
            offset: -18000,
            format: "ECT",
            saving: Saving::NoSaving,
            start_time: Some(-1230768000),
            end_time:   Some(504921600),
        },
        Timespan {
            offset: -21600,
            format: "GALT",
            saving: Saving::NoSaving,
            start_time: Some(504921600),
            end_time:   None,
        },
    ],
};

