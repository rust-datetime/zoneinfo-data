
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Antarctica/Palmer",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   "zzz",
        },
        rest: &[
        (-157766400, FixedTimespan {  // 1965-00-01T0-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "ARST",
        }),
        (-152658000, FixedTimespan {  // 1965-02-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "ART",
        }),
        (-132955200, FixedTimespan {  // 1965-09-15T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "ARST",
        }),
        (-121122000, FixedTimespan {  // 1966-02-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "ART",
        }),
        (-101419200, FixedTimespan {  // 1966-09-15T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "ARST",
        }),
        (-86821200, FixedTimespan {  // 1967-03-02T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "ART",
        }),
        (-71092800, FixedTimespan {  // 1967-09-01T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "ARST",
        }),
        (-54766800, FixedTimespan {  // 1968-03-07T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "ART",
        }),
        (-39038400, FixedTimespan {  // 1968-09-06T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "ARST",
        }),
        (-23317200, FixedTimespan {  // 1969-03-06T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "ART",
        }),
        (-7588800, FixedTimespan {  // 1969-09-05T4-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "ART",
        }),
        (128142000, FixedTimespan {  // 1974-00-23T3-00-00 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   "ARST",
        }),
        (136605600, FixedTimespan {  // 1974-04-01T2-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "ART",
        }),
        (389070000, FixedTimespan {  // 1982-04-01T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (403070400, FixedTimespan {  // 1982-09-10T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (416372400, FixedTimespan {  // 1983-02-13T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (434520000, FixedTimespan {  // 1983-09-09T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (447822000, FixedTimespan {  // 1984-02-11T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (466574400, FixedTimespan {  // 1984-09-14T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (479271600, FixedTimespan {  // 1985-02-10T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (498024000, FixedTimespan {  // 1985-09-13T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (510721200, FixedTimespan {  // 1986-02-09T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (529473600, FixedTimespan {  // 1986-09-12T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (545194800, FixedTimespan {  // 1987-03-12T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (560923200, FixedTimespan {  // 1987-09-11T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (574225200, FixedTimespan {  // 1988-02-13T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (592372800, FixedTimespan {  // 1988-09-09T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (605674800, FixedTimespan {  // 1989-02-12T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (624427200, FixedTimespan {  // 1989-09-15T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (637124400, FixedTimespan {  // 1990-02-11T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (653457600, FixedTimespan {  // 1990-08-16T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (668574000, FixedTimespan {  // 1991-02-10T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (687326400, FixedTimespan {  // 1991-09-13T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (700628400, FixedTimespan {  // 1992-02-15T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (718776000, FixedTimespan {  // 1992-09-11T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (732078000, FixedTimespan {  // 1993-02-14T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (750225600, FixedTimespan {  // 1993-09-10T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (763527600, FixedTimespan {  // 1994-02-13T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (781675200, FixedTimespan {  // 1994-09-09T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (794977200, FixedTimespan {  // 1995-02-12T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (813729600, FixedTimespan {  // 1995-09-15T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (826426800, FixedTimespan {  // 1996-02-10T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (845179200, FixedTimespan {  // 1996-09-13T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (859690800, FixedTimespan {  // 1997-02-30T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (876628800, FixedTimespan {  // 1997-09-12T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (889930800, FixedTimespan {  // 1998-02-15T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (906868800, FixedTimespan {  // 1998-08-27T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (923194800, FixedTimespan {  // 1999-03-04T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (939528000, FixedTimespan {  // 1999-09-10T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (952830000, FixedTimespan {  // 2000-02-12T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (971582400, FixedTimespan {  // 2000-09-15T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (984279600, FixedTimespan {  // 2001-02-11T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1003032000, FixedTimespan {  // 2001-09-14T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1015729200, FixedTimespan {  // 2002-02-10T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1034481600, FixedTimespan {  // 2002-09-13T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1047178800, FixedTimespan {  // 2003-02-09T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1065931200, FixedTimespan {  // 2003-09-12T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1079233200, FixedTimespan {  // 2004-02-14T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1097380800, FixedTimespan {  // 2004-09-10T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1110682800, FixedTimespan {  // 2005-02-13T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1128830400, FixedTimespan {  // 2005-09-09T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1142132400, FixedTimespan {  // 2006-02-12T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1160884800, FixedTimespan {  // 2006-09-15T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1173582000, FixedTimespan {  // 2007-02-11T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1192334400, FixedTimespan {  // 2007-09-14T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1206846000, FixedTimespan {  // 2008-02-30T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1223784000, FixedTimespan {  // 2008-09-12T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1237086000, FixedTimespan {  // 2009-02-15T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1255233600, FixedTimespan {  // 2009-09-11T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1270350000, FixedTimespan {  // 2010-03-04T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1286683200, FixedTimespan {  // 2010-09-10T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1304823600, FixedTimespan {  // 2011-04-08T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1313899200, FixedTimespan {  // 2011-07-21T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1335668400, FixedTimespan {  // 2012-03-29T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1346558400, FixedTimespan {  // 2012-08-02T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1367118000, FixedTimespan {  // 2013-03-28T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1378612800, FixedTimespan {  // 2013-08-08T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1398567600, FixedTimespan {  // 2014-03-27T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1410062400, FixedTimespan {  // 2014-08-07T4-00-00 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   "CLST",
        }),
        (1430017200, FixedTimespan {  // 2015-03-26T3-00-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
        (1430031600, FixedTimespan {  // 2015-03-26T7-00-00 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   "CLT",
        }),
    ]},
};


