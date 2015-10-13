
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Norfolk",
    timespans: &[
        Timespan {
            offset: 40312,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2177452800),
        },
        Timespan {
            offset: 40320,
            format: "NMT",
            saving: Saving::NoSaving,
            start_time: Some(-2177452800),
            end_time:   Some(-599616000),
        },
        Timespan {
            offset: 41400,
            format: "NFT",
            saving: Saving::NoSaving,
            start_time: Some(-599616000),
            end_time:   Some(152071200),
        },
        Timespan {
            offset: 41400,
            format: "NFST",
            saving: Saving::OneOff(3600),
            start_time: Some(152071200),
            end_time:   Some(162957600),
        },
        Timespan {
            offset: 41400,
            format: "NFT",
            saving: Saving::NoSaving,
            start_time: Some(162957600),
            end_time:   Some(1443924000),
        },
        Timespan {
            offset: 39600,
            format: "NFT",
            saving: Saving::NoSaving,
            start_time: Some(1443924000),
            end_time:   None,
        },
    ],
};

