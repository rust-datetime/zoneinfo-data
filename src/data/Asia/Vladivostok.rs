
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Vladivostok",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 31651,  // UTC offset 31651, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1487321251, FixedTimespan {  // 1922-10-14T15-12-29 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (-1247562000, FixedTimespan {  // 1930-05-20T15-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (354895200, FixedTimespan {  // 1981-02-31T14-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (370702800, FixedTimespan {  // 1981-08-30T13-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (386431200, FixedTimespan {  // 1982-02-31T14-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (402238800, FixedTimespan {  // 1982-08-30T13-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (417967200, FixedTimespan {  // 1983-02-31T14-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (433774800, FixedTimespan {  // 1983-08-30T13-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (449589600, FixedTimespan {  // 1984-02-31T14-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (465321600, FixedTimespan {  // 1984-08-29T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (481046400, FixedTimespan {  // 1985-02-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (496771200, FixedTimespan {  // 1985-08-28T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (512496000, FixedTimespan {  // 1986-02-29T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (528220800, FixedTimespan {  // 1986-08-27T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (543945600, FixedTimespan {  // 1987-02-28T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (559670400, FixedTimespan {  // 1987-08-26T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (575395200, FixedTimespan {  // 1988-02-26T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (591120000, FixedTimespan {  // 1988-08-24T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (606844800, FixedTimespan {  // 1989-02-25T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (622569600, FixedTimespan {  // 1989-08-23T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (638294400, FixedTimespan {  // 1990-02-24T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (654624000, FixedTimespan {  // 1990-08-29T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (670348800, FixedTimespan {  // 1991-02-30T16-00-00 UTC
            offset: 36000,  // UTC offset 32400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (686077200, FixedTimespan {  // 1991-08-28T17-00-00 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (695754000, FixedTimespan {  // 1992-00-18T17-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (701787600, FixedTimespan {  // 1992-02-28T13-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (717508800, FixedTimespan {  // 1992-08-26T12-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (733248000, FixedTimespan {  // 1993-02-27T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (748972800, FixedTimespan {  // 1993-08-25T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (764697600, FixedTimespan {  // 1994-02-26T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (780422400, FixedTimespan {  // 1994-08-24T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (796147200, FixedTimespan {  // 1995-02-25T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (811872000, FixedTimespan {  // 1995-08-23T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (828201600, FixedTimespan {  // 1996-02-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (846345600, FixedTimespan {  // 1996-09-26T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (859651200, FixedTimespan {  // 1997-02-29T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (877795200, FixedTimespan {  // 1997-09-25T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (891100800, FixedTimespan {  // 1998-02-28T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (909244800, FixedTimespan {  // 1998-09-24T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (922550400, FixedTimespan {  // 1999-02-27T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (941299200, FixedTimespan {  // 1999-09-30T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (954000000, FixedTimespan {  // 2000-02-25T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (972748800, FixedTimespan {  // 2000-09-28T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (985449600, FixedTimespan {  // 2001-02-24T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1004198400, FixedTimespan {  // 2001-09-27T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1017504000, FixedTimespan {  // 2002-02-30T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1035648000, FixedTimespan {  // 2002-09-26T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1048953600, FixedTimespan {  // 2003-02-29T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1067097600, FixedTimespan {  // 2003-09-25T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1080403200, FixedTimespan {  // 2004-02-27T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1099152000, FixedTimespan {  // 2004-09-30T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1111852800, FixedTimespan {  // 2005-02-26T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1130601600, FixedTimespan {  // 2005-09-29T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1143302400, FixedTimespan {  // 2006-02-25T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1162051200, FixedTimespan {  // 2006-09-28T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1174752000, FixedTimespan {  // 2007-02-24T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1193500800, FixedTimespan {  // 2007-09-27T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1206806400, FixedTimespan {  // 2008-02-29T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1224950400, FixedTimespan {  // 2008-09-25T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1238256000, FixedTimespan {  // 2009-02-28T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1256400000, FixedTimespan {  // 2009-09-24T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1269705600, FixedTimespan {  // 2010-02-27T16-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1288454400, FixedTimespan {  // 2010-09-30T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1301155200, FixedTimespan {  // 2011-02-26T16-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1414249200, FixedTimespan {  // 2014-09-25T15-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
    ]},
};


