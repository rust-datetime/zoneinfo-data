
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Yerevan",
    timespans: &[
        Timespan {
            offset: 10680,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1441152000),
        },
        Timespan {
            offset: 10800,
            format: "YERT",
            saving: Saving::NoSaving,
            start_time: Some(-1441152000),
            end_time:   Some(-405129600),
        },
        Timespan {
            offset: 14400,
            format: "YER%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(-405129600),
            end_time:   Some(670384800),
        },
        Timespan {
            offset: 10800,
            format: "YERST",
            saving: Saving::OneOff(3600),
            start_time: Some(670384800),
            end_time:   Some(685584000),
        },
        Timespan {
            offset: 10800,
            format: "AM%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(685584000),
            end_time:   Some(811908000),
        },
        Timespan {
            offset: 14400,
            format: "AMT",
            saving: Saving::NoSaving,
            start_time: Some(811908000),
            end_time:   Some(852076800),
        },
        Timespan {
            offset: 14400,
            format: "AM%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(852076800),
            end_time:   Some(1332640800),
        },
        Timespan {
            offset: 14400,
            format: "AMT",
            saving: Saving::NoSaving,
            start_time: Some(1332640800),
            end_time:   None,
        },
    ],
};

