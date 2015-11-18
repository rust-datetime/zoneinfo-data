
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Europe/Minsk",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 6616,  // UTC offset 6616, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2840147416, FixedTimespan {  // 1879-11-31T22-09-44 UTC
            offset: 6600,  // UTC offset 6600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MMT"),
        }),
        (-1441158600, FixedTimespan {  // 1924-04-01T22-10-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (-1247536800, FixedTimespan {  // 1930-05-20T22-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (-899780400, FixedTimespan {  // 1941-05-27T21-00-00 UTC
            offset: 7200,  // UTC offset 3600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CEST"),
        }),
        (-857257200, FixedTimespan {  // 1942-10-02T1-00-00 UTC
            offset: 3600,  // UTC offset 3600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CET"),
        }),
        (-844556400, FixedTimespan {  // 1943-02-29T1-00-00 UTC
            offset: 7200,  // UTC offset 3600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CEST"),
        }),
        (-828226800, FixedTimespan {  // 1943-09-04T1-00-00 UTC
            offset: 3600,  // UTC offset 3600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CET"),
        }),
        (-812502000, FixedTimespan {  // 1944-03-03T1-00-00 UTC
            offset: 7200,  // UTC offset 3600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CEST"),
        }),
        (-804650400, FixedTimespan {  // 1944-06-02T22-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (354920400, FixedTimespan {  // 1981-02-31T21-00-00 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MSD"),
        }),
        (370728000, FixedTimespan {  // 1981-08-30T20-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (386456400, FixedTimespan {  // 1982-02-31T21-00-00 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MSD"),
        }),
        (402264000, FixedTimespan {  // 1982-08-30T20-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (417992400, FixedTimespan {  // 1983-02-31T21-00-00 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MSD"),
        }),
        (433800000, FixedTimespan {  // 1983-08-30T20-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (449614800, FixedTimespan {  // 1984-02-31T21-00-00 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MSD"),
        }),
        (465346800, FixedTimespan {  // 1984-08-29T23-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (481071600, FixedTimespan {  // 1985-02-30T23-00-00 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MSD"),
        }),
        (496796400, FixedTimespan {  // 1985-08-28T23-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (512521200, FixedTimespan {  // 1986-02-29T23-00-00 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MSD"),
        }),
        (528246000, FixedTimespan {  // 1986-08-27T23-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (543970800, FixedTimespan {  // 1987-02-28T23-00-00 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MSD"),
        }),
        (559695600, FixedTimespan {  // 1987-08-26T23-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (575420400, FixedTimespan {  // 1988-02-26T23-00-00 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MSD"),
        }),
        (591145200, FixedTimespan {  // 1988-08-24T23-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (606870000, FixedTimespan {  // 1989-02-25T23-00-00 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MSD"),
        }),
        (622594800, FixedTimespan {  // 1989-08-23T23-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
        (670374000, FixedTimespan {  // 1991-02-30T23-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (686098800, FixedTimespan {  // 1991-08-28T23-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (701820000, FixedTimespan {  // 1992-02-28T22-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (717541200, FixedTimespan {  // 1992-08-26T21-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (733276800, FixedTimespan {  // 1993-02-28T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (749001600, FixedTimespan {  // 1993-08-26T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (764726400, FixedTimespan {  // 1994-02-27T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (780451200, FixedTimespan {  // 1994-08-25T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (796176000, FixedTimespan {  // 1995-02-26T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (811900800, FixedTimespan {  // 1995-08-24T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (828230400, FixedTimespan {  // 1996-02-31T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (846374400, FixedTimespan {  // 1996-09-27T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (859680000, FixedTimespan {  // 1997-02-30T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (877824000, FixedTimespan {  // 1997-09-26T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (891129600, FixedTimespan {  // 1998-02-29T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (909273600, FixedTimespan {  // 1998-09-25T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (922579200, FixedTimespan {  // 1999-02-28T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (941328000, FixedTimespan {  // 1999-09-31T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (954028800, FixedTimespan {  // 2000-02-26T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (972777600, FixedTimespan {  // 2000-09-29T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (985478400, FixedTimespan {  // 2001-02-25T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1004227200, FixedTimespan {  // 2001-09-28T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1017532800, FixedTimespan {  // 2002-02-31T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1035676800, FixedTimespan {  // 2002-09-27T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1048982400, FixedTimespan {  // 2003-02-30T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1067126400, FixedTimespan {  // 2003-09-26T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1080432000, FixedTimespan {  // 2004-02-28T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1099180800, FixedTimespan {  // 2004-09-31T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1111881600, FixedTimespan {  // 2005-02-27T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1130630400, FixedTimespan {  // 2005-09-30T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1143331200, FixedTimespan {  // 2006-02-26T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1162080000, FixedTimespan {  // 2006-09-29T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1174780800, FixedTimespan {  // 2007-02-25T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1193529600, FixedTimespan {  // 2007-09-28T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1206835200, FixedTimespan {  // 2008-02-30T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1224979200, FixedTimespan {  // 2008-09-26T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1238284800, FixedTimespan {  // 2009-02-29T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1256428800, FixedTimespan {  // 2009-09-25T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1269734400, FixedTimespan {  // 2010-02-28T0-00-00 UTC
            offset: 10800,  // UTC offset 7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EEST"),
        }),
        (1288483200, FixedTimespan {  // 2010-09-31T0-00-00 UTC
            offset: 7200,  // UTC offset 7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EET"),
        }),
        (1301184000, FixedTimespan {  // 2011-02-27T0-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FET"),
        }),
        (1414274400, FixedTimespan {  // 2014-09-25T22-00-00 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MSK"),
        }),
    ]},
};


