
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Tongatapu",
    timespans: &[
        Timespan {
            offset: 44360,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2177452800),
        },
        Timespan {
            offset: 44400,
            format: "TOT",
            saving: Saving::NoSaving,
            start_time: Some(-2177452800),
            end_time:   Some(-915148800),
        },
        Timespan {
            offset: 46800,
            format: "TOT",
            saving: Saving::NoSaving,
            start_time: Some(-915148800),
            end_time:   Some(915148800),
        },
        Timespan {
            offset: 46800,
            format: "TO%sT",
            saving: Saving::Multiple(&rulesets::Tonga),
            start_time: Some(915148800),
            end_time:   None,
        },
    ],
};

