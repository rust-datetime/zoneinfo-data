
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Ho_Chi_Minh",
    timespans: &[
        Timespan {
            offset: 25600,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2004048000),
        },
        Timespan {
            offset: 25590,
            format: "PLMT",
            saving: Saving::NoSaving,
            start_time: Some(-2004048000),
            end_time:   Some(-1851552000),
        },
        Timespan {
            offset: 25200,
            format: "ICT",
            saving: Saving::NoSaving,
            start_time: Some(-1851552000),
            end_time:   Some(-852080400),
        },
        Timespan {
            offset: 28800,
            format: "IDT",
            saving: Saving::NoSaving,
            start_time: Some(-852080400),
            end_time:   Some(-782614800),
        },
        Timespan {
            offset: 32400,
            format: "JST",
            saving: Saving::NoSaving,
            start_time: Some(-782614800),
            end_time:   Some(-767836800),
        },
        Timespan {
            offset: 25200,
            format: "ICT",
            saving: Saving::NoSaving,
            start_time: Some(-767836800),
            end_time:   Some(-718070400),
        },
        Timespan {
            offset: 28800,
            format: "IDT",
            saving: Saving::NoSaving,
            start_time: Some(-718070400),
            end_time:   Some(-457747200),
        },
        Timespan {
            offset: 25200,
            format: "ICT",
            saving: Saving::NoSaving,
            start_time: Some(-457747200),
            end_time:   Some(-315622800),
        },
        Timespan {
            offset: 28800,
            format: "IDT",
            saving: Saving::NoSaving,
            start_time: Some(-315622800),
            end_time:   Some(171849600),
        },
        Timespan {
            offset: 25200,
            format: "ICT",
            saving: Saving::NoSaving,
            start_time: Some(171849600),
            end_time:   None,
        },
    ],
};


