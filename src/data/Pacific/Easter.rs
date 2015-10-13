
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Easter",
    timespans: &[
        Timespan {
            offset: -24152,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2524521600),
        },
        Timespan {
            offset: -24152,
            format: "EMT",
            saving: Saving::NoSaving,
            start_time: Some(-2524521600),
            end_time:   Some(-1178150400),
        },
        Timespan {
            offset: -25200,
            format: "EAS%sT",
            saving: Saving::Multiple(&rulesets::Chile),
            start_time: Some(-1178150400),
            end_time:   Some(384922800),
        },
        Timespan {
            offset: -21600,
            format: "EAS%sT",
            saving: Saving::Multiple(&rulesets::Chile),
            start_time: Some(384922800),
            end_time:   Some(1430017200),
        },
        Timespan {
            offset: -18000,
            format: "EAST",
            saving: Saving::NoSaving,
            start_time: Some(1430017200),
            end_time:   None,
        },
    ],
};

