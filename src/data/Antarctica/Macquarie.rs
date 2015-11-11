
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Antarctica/Macquarie",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   "zzz",
        },
        rest: &[
        (-2214259200, FixedTimespan {  // 1899-10-01T0-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (-1680508800, FixedTimespan {  // 1916-08-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (-1665392400, FixedTimespan {  // 1917-02-24T15-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (-1601719200, FixedTimespan {  // 1919-02-31T14-00-00 UTC
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   "zzz",
        }),
        (-687052800, FixedTimespan {  // 1948-02-25T0-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (-71136000, FixedTimespan {  // 1967-08-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (-55411200, FixedTimespan {  // 1968-02-30T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (-37267200, FixedTimespan {  // 1968-09-26T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (-25776000, FixedTimespan {  // 1969-02-08T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (-5817600, FixedTimespan {  // 1969-09-25T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (5673600, FixedTimespan {  // 1970-02-07T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (25632000, FixedTimespan {  // 1970-09-24T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (37728000, FixedTimespan {  // 1971-02-13T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (57686400, FixedTimespan {  // 1971-09-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (67968000, FixedTimespan {  // 1972-01-26T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (89136000, FixedTimespan {  // 1972-09-28T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (100022400, FixedTimespan {  // 1973-02-03T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (120585600, FixedTimespan {  // 1973-09-27T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (131472000, FixedTimespan {  // 1974-02-02T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (152035200, FixedTimespan {  // 1974-09-26T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (162921600, FixedTimespan {  // 1975-02-01T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (183484800, FixedTimespan {  // 1975-09-25T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (194976000, FixedTimespan {  // 1976-02-06T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (215539200, FixedTimespan {  // 1976-09-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (226425600, FixedTimespan {  // 1977-02-05T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (246988800, FixedTimespan {  // 1977-09-29T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (257875200, FixedTimespan {  // 1978-02-04T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (278438400, FixedTimespan {  // 1978-09-28T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (289324800, FixedTimespan {  // 1979-02-03T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (309888000, FixedTimespan {  // 1979-09-27T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (320774400, FixedTimespan {  // 1980-02-01T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (341337600, FixedTimespan {  // 1980-09-25T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (352224000, FixedTimespan {  // 1981-01-28T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (372787200, FixedTimespan {  // 1981-09-24T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (386092800, FixedTimespan {  // 1982-02-27T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (404841600, FixedTimespan {  // 1982-09-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (417542400, FixedTimespan {  // 1983-02-26T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (436291200, FixedTimespan {  // 1983-09-29T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (447177600, FixedTimespan {  // 1984-02-03T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (467740800, FixedTimespan {  // 1984-09-27T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (478627200, FixedTimespan {  // 1985-02-02T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (499190400, FixedTimespan {  // 1985-09-26T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (510076800, FixedTimespan {  // 1986-02-01T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (530035200, FixedTimespan {  // 1986-09-18T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (542736000, FixedTimespan {  // 1987-02-14T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (562089600, FixedTimespan {  // 1987-09-24T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (574790400, FixedTimespan {  // 1988-02-19T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (594144000, FixedTimespan {  // 1988-09-29T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (606240000, FixedTimespan {  // 1989-02-18T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (625593600, FixedTimespan {  // 1989-09-28T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (637689600, FixedTimespan {  // 1990-02-17T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (657043200, FixedTimespan {  // 1990-09-27T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (670348800, FixedTimespan {  // 1991-02-30T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (686678400, FixedTimespan {  // 1991-09-05T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (701798400, FixedTimespan {  // 1992-02-28T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (718128000, FixedTimespan {  // 1992-09-03T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (733248000, FixedTimespan {  // 1993-02-27T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (749577600, FixedTimespan {  // 1993-09-02T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (764697600, FixedTimespan {  // 1994-02-26T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (781027200, FixedTimespan {  // 1994-09-01T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (796147200, FixedTimespan {  // 1995-02-25T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (812476800, FixedTimespan {  // 1995-08-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (828201600, FixedTimespan {  // 1996-02-30T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (844531200, FixedTimespan {  // 1996-09-05T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (859651200, FixedTimespan {  // 1997-02-29T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (875980800, FixedTimespan {  // 1997-09-04T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (891100800, FixedTimespan {  // 1998-02-28T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (907430400, FixedTimespan {  // 1998-09-03T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (922550400, FixedTimespan {  // 1999-02-27T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (938880000, FixedTimespan {  // 1999-09-02T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (954000000, FixedTimespan {  // 2000-02-25T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (967305600, FixedTimespan {  // 2000-07-26T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (985449600, FixedTimespan {  // 2001-02-24T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (1002384000, FixedTimespan {  // 2001-09-06T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (1017504000, FixedTimespan {  // 2002-02-30T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (1033833600, FixedTimespan {  // 2002-09-05T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (1048953600, FixedTimespan {  // 2003-02-29T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (1065283200, FixedTimespan {  // 2003-09-04T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (1080403200, FixedTimespan {  // 2004-02-27T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (1096732800, FixedTimespan {  // 2004-09-02T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (1111852800, FixedTimespan {  // 2005-02-26T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (1128182400, FixedTimespan {  // 2005-09-01T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (1143907200, FixedTimespan {  // 2006-03-01T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (1159632000, FixedTimespan {  // 2006-08-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (1174752000, FixedTimespan {  // 2007-02-24T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (1191686400, FixedTimespan {  // 2007-09-06T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (1207411200, FixedTimespan {  // 2008-03-05T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (1223136000, FixedTimespan {  // 2008-09-04T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (1238860800, FixedTimespan {  // 2009-03-04T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "AEST",
        }),
        (1254585600, FixedTimespan {  // 2009-09-03T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "AEDT",
        }),
        (1270310400, FixedTimespan {  // 2010-03-03T16-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MIST",
        }),
    ]},
};


