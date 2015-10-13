
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Atlantic/Reykjavik",
    timespans: &[
        Timespan {
            offset: -1920,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1956614400),
        },
        Timespan {
            offset: -3600,
            format: "IS%sT",
            saving: Saving::Multiple(&rulesets::Iceland),
            start_time: Some(-1956614400),
            end_time:   Some(-54774000),
        },
        Timespan {
            offset: 0,
            format: "GMT",
            saving: Saving::NoSaving,
            start_time: Some(-54774000),
            end_time:   None,
        },
    ],
};


