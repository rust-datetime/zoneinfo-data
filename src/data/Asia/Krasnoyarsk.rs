
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Krasnoyarsk",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 22286,  // UTC offset 22286, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1577513486, FixedTimespan {  // 1920-01-05T17:48:34.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (-1247551200, FixedTimespan {  // 1930-06-20T18:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (354906000, FixedTimespan {  // 1981-03-31T17:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (370713600, FixedTimespan {  // 1981-09-30T16:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (386442000, FixedTimespan {  // 1982-03-31T17:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (402249600, FixedTimespan {  // 1982-09-30T16:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (417978000, FixedTimespan {  // 1983-03-31T17:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (433785600, FixedTimespan {  // 1983-09-30T16:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (449600400, FixedTimespan {  // 1984-03-31T17:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (465332400, FixedTimespan {  // 1984-09-29T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (481057200, FixedTimespan {  // 1985-03-30T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (496782000, FixedTimespan {  // 1985-09-28T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (512506800, FixedTimespan {  // 1986-03-29T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (528231600, FixedTimespan {  // 1986-09-27T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (543956400, FixedTimespan {  // 1987-03-28T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (559681200, FixedTimespan {  // 1987-09-26T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (575406000, FixedTimespan {  // 1988-03-26T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (591130800, FixedTimespan {  // 1988-09-24T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (606855600, FixedTimespan {  // 1989-03-25T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (622580400, FixedTimespan {  // 1989-09-23T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (638305200, FixedTimespan {  // 1990-03-24T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (654634800, FixedTimespan {  // 1990-09-29T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (670359600, FixedTimespan {  // 1991-03-30T19:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (686088000, FixedTimespan {  // 1991-09-28T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (695764800, FixedTimespan {  // 1992-01-18T20:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (701798400, FixedTimespan {  // 1992-03-28T16:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (717519600, FixedTimespan {  // 1992-09-26T15:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (733258800, FixedTimespan {  // 1993-03-27T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (748983600, FixedTimespan {  // 1993-09-25T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (764708400, FixedTimespan {  // 1994-03-26T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (780433200, FixedTimespan {  // 1994-09-24T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (796158000, FixedTimespan {  // 1995-03-25T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (811882800, FixedTimespan {  // 1995-09-23T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (828212400, FixedTimespan {  // 1996-03-30T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (846356400, FixedTimespan {  // 1996-10-26T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (859662000, FixedTimespan {  // 1997-03-29T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (877806000, FixedTimespan {  // 1997-10-25T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (891111600, FixedTimespan {  // 1998-03-28T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (909255600, FixedTimespan {  // 1998-10-24T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (922561200, FixedTimespan {  // 1999-03-27T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (941310000, FixedTimespan {  // 1999-10-30T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (954010800, FixedTimespan {  // 2000-03-25T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (972759600, FixedTimespan {  // 2000-10-28T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (985460400, FixedTimespan {  // 2001-03-24T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1004209200, FixedTimespan {  // 2001-10-27T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1017514800, FixedTimespan {  // 2002-03-30T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1035658800, FixedTimespan {  // 2002-10-26T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1048964400, FixedTimespan {  // 2003-03-29T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1067108400, FixedTimespan {  // 2003-10-25T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1080414000, FixedTimespan {  // 2004-03-27T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1099162800, FixedTimespan {  // 2004-10-30T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1111863600, FixedTimespan {  // 2005-03-26T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1130612400, FixedTimespan {  // 2005-10-29T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1143313200, FixedTimespan {  // 2006-03-25T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1162062000, FixedTimespan {  // 2006-10-28T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1174762800, FixedTimespan {  // 2007-03-24T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1193511600, FixedTimespan {  // 2007-10-27T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1206817200, FixedTimespan {  // 2008-03-29T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1224961200, FixedTimespan {  // 2008-10-25T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1238266800, FixedTimespan {  // 2009-03-28T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1256410800, FixedTimespan {  // 2009-10-24T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1269716400, FixedTimespan {  // 2010-03-27T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KRAST"),
        }),
        (1288465200, FixedTimespan {  // 2010-10-30T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1301166000, FixedTimespan {  // 2011-03-26T19:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
        (1414260000, FixedTimespan {  // 2014-10-25T18:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KRAT"),
        }),
    ]},
};


