
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Cancun",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -15176,  // UTC offset -15176, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1514748848, FixedTimespan {  // 1922-00-01T4-25-52 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (377935200, FixedTimespan {  // 1981-11-23T6-00-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (828860400, FixedTimespan {  // 1996-03-07T7-00-00 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (846396000, FixedTimespan {  // 1996-09-27T6-00-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (860310000, FixedTimespan {  // 1997-03-06T7-00-00 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (877845600, FixedTimespan {  // 1997-09-26T6-00-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (891759600, FixedTimespan {  // 1998-03-05T7-00-00 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (902037600, FixedTimespan {  // 1998-07-02T6-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (909298800, FixedTimespan {  // 1998-09-25T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (923212800, FixedTimespan {  // 1999-03-04T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (941353200, FixedTimespan {  // 1999-09-31T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (954662400, FixedTimespan {  // 2000-03-02T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (972802800, FixedTimespan {  // 2000-09-29T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (989136000, FixedTimespan {  // 2001-04-06T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1001833200, FixedTimespan {  // 2001-08-30T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1018166400, FixedTimespan {  // 2002-03-07T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1035702000, FixedTimespan {  // 2002-09-27T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1049616000, FixedTimespan {  // 2003-03-06T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1067151600, FixedTimespan {  // 2003-09-26T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1081065600, FixedTimespan {  // 2004-03-04T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1099206000, FixedTimespan {  // 2004-09-31T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1112515200, FixedTimespan {  // 2005-03-03T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1130655600, FixedTimespan {  // 2005-09-30T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1143964800, FixedTimespan {  // 2006-03-02T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1162105200, FixedTimespan {  // 2006-09-29T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1175414400, FixedTimespan {  // 2007-03-01T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1193554800, FixedTimespan {  // 2007-09-28T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1207468800, FixedTimespan {  // 2008-03-06T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1225004400, FixedTimespan {  // 2008-09-26T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1238918400, FixedTimespan {  // 2009-03-05T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1256454000, FixedTimespan {  // 2009-09-25T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1270368000, FixedTimespan {  // 2010-03-04T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1288508400, FixedTimespan {  // 2010-09-31T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1301817600, FixedTimespan {  // 2011-03-03T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1319958000, FixedTimespan {  // 2011-09-30T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1333267200, FixedTimespan {  // 2012-03-01T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1351407600, FixedTimespan {  // 2012-09-28T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1365321600, FixedTimespan {  // 2013-03-07T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1382857200, FixedTimespan {  // 2013-09-27T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1396771200, FixedTimespan {  // 2014-03-06T8-00-00 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (1414306800, FixedTimespan {  // 2014-09-26T7-00-00 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (1422777600, FixedTimespan {  // 2015-01-01T8-00-00 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
    ]},
};


