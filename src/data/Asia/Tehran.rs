
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Tehran",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 12344,  // UTC offset 12344, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1704165944, FixedTimespan {  // 1915-11-31T20-34-16 UTC
            offset: 12344,  // UTC offset 12344, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("TMT"),
        }),
        (-757394744, FixedTimespan {  // 1945-11-31T20-34-16 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (247177800, FixedTimespan {  // 1977-09-31T20-30-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (259272000, FixedTimespan {  // 1978-02-20T20-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (277758000, FixedTimespan {  // 1978-09-20T19-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (283982400, FixedTimespan {  // 1978-11-31T20-00-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (290809800, FixedTimespan {  // 1979-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (306531000, FixedTimespan {  // 1979-08-18T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (322432200, FixedTimespan {  // 1980-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (338499000, FixedTimespan {  // 1980-08-22T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (673216200, FixedTimespan {  // 1991-04-02T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (685481400, FixedTimespan {  // 1991-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (701209800, FixedTimespan {  // 1992-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (717103800, FixedTimespan {  // 1992-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (732745800, FixedTimespan {  // 1993-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (748639800, FixedTimespan {  // 1993-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (764281800, FixedTimespan {  // 1994-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (780175800, FixedTimespan {  // 1994-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (795817800, FixedTimespan {  // 1995-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (811711800, FixedTimespan {  // 1995-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (827353800, FixedTimespan {  // 1996-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (843247800, FixedTimespan {  // 1996-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (858976200, FixedTimespan {  // 1997-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (874870200, FixedTimespan {  // 1997-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (890512200, FixedTimespan {  // 1998-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (906406200, FixedTimespan {  // 1998-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (922048200, FixedTimespan {  // 1999-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (937942200, FixedTimespan {  // 1999-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (953584200, FixedTimespan {  // 2000-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (969478200, FixedTimespan {  // 2000-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (985206600, FixedTimespan {  // 2001-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1001100600, FixedTimespan {  // 2001-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1016742600, FixedTimespan {  // 2002-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1032636600, FixedTimespan {  // 2002-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1048278600, FixedTimespan {  // 2003-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1064172600, FixedTimespan {  // 2003-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1079814600, FixedTimespan {  // 2004-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1095708600, FixedTimespan {  // 2004-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1111437000, FixedTimespan {  // 2005-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1127331000, FixedTimespan {  // 2005-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1206045000, FixedTimespan {  // 2008-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1221939000, FixedTimespan {  // 2008-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1237667400, FixedTimespan {  // 2009-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1253561400, FixedTimespan {  // 2009-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1269203400, FixedTimespan {  // 2010-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1285097400, FixedTimespan {  // 2010-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1300739400, FixedTimespan {  // 2011-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1316633400, FixedTimespan {  // 2011-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1332275400, FixedTimespan {  // 2012-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1348169400, FixedTimespan {  // 2012-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1363897800, FixedTimespan {  // 2013-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1379791800, FixedTimespan {  // 2013-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1395433800, FixedTimespan {  // 2014-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1411327800, FixedTimespan {  // 2014-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1426969800, FixedTimespan {  // 2015-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1442863800, FixedTimespan {  // 2015-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1458505800, FixedTimespan {  // 2016-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1474399800, FixedTimespan {  // 2016-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1490128200, FixedTimespan {  // 2017-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1506022200, FixedTimespan {  // 2017-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1521664200, FixedTimespan {  // 2018-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1537558200, FixedTimespan {  // 2018-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1553200200, FixedTimespan {  // 2019-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1569094200, FixedTimespan {  // 2019-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1584736200, FixedTimespan {  // 2020-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1600630200, FixedTimespan {  // 2020-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1616358600, FixedTimespan {  // 2021-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1632252600, FixedTimespan {  // 2021-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1647894600, FixedTimespan {  // 2022-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1663788600, FixedTimespan {  // 2022-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1679430600, FixedTimespan {  // 2023-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1695324600, FixedTimespan {  // 2023-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1710966600, FixedTimespan {  // 2024-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1726860600, FixedTimespan {  // 2024-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1742589000, FixedTimespan {  // 2025-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1758483000, FixedTimespan {  // 2025-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1774125000, FixedTimespan {  // 2026-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1790019000, FixedTimespan {  // 2026-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1805661000, FixedTimespan {  // 2027-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1821555000, FixedTimespan {  // 2027-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1837197000, FixedTimespan {  // 2028-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1853091000, FixedTimespan {  // 2028-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1868733000, FixedTimespan {  // 2029-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1884627000, FixedTimespan {  // 2029-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1900355400, FixedTimespan {  // 2030-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1916249400, FixedTimespan {  // 2030-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1931891400, FixedTimespan {  // 2031-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1947785400, FixedTimespan {  // 2031-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1963427400, FixedTimespan {  // 2032-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (1979321400, FixedTimespan {  // 2032-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (1994963400, FixedTimespan {  // 2033-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (2010857400, FixedTimespan {  // 2033-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (2026585800, FixedTimespan {  // 2034-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (2042479800, FixedTimespan {  // 2034-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (2058121800, FixedTimespan {  // 2035-02-21T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (2074015800, FixedTimespan {  // 2035-08-21T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (2089657800, FixedTimespan {  // 2036-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (2105551800, FixedTimespan {  // 2036-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
        (2121193800, FixedTimespan {  // 2037-02-20T20-30-00 UTC
            offset: 16200,  // UTC offset 12600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("IRDT"),
        }),
        (2137087800, FixedTimespan {  // 2037-08-20T19-30-00 UTC
            offset: 12600,  // UTC offset 12600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IRST"),
        }),
    ]},
};


