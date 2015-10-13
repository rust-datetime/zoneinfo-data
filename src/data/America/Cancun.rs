
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Cancun",
    timespans: &[
        Timespan {
            offset: -15176,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1514764024),
        },
        Timespan {
            offset: -21600,
            format: "CST",
            saving: Saving::NoSaving,
            start_time: Some(-1514764024),
            end_time:   Some(377913600),
        },
        Timespan {
            offset: -18000,
            format: "E%sT",
            saving: Saving::Multiple(&rulesets::Mexico),
            start_time: Some(377913600),
            end_time:   Some(902023200),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::Mexico),
            start_time: Some(902023200),
            end_time:   Some(1422756000),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(1422756000),
            end_time:   None,
        },
    ],
};


