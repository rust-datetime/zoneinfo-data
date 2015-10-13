
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/North_Dakota/New_Salem",
    timespans: &[
        Timespan {
            offset: -18861,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2717667939),
        },
        Timespan {
            offset: -25200,
            format: "M%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(-2717667939),
            end_time:   Some(1067133600),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(1067133600),
            end_time:   None,
        },
    ],
};

