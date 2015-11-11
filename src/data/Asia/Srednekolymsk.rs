
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Asia/Srednekolymsk",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 36892,  // UTC offset 36892, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-1441188892, FixedTimespan {  // 1924-04-01T13-45-08 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (-1247565600, FixedTimespan {  // 1930-05-20T14-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (354891600, FixedTimespan {  // 1981-02-31T13-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (370699200, FixedTimespan {  // 1981-08-30T12-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (386427600, FixedTimespan {  // 1982-02-31T13-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (402235200, FixedTimespan {  // 1982-08-30T12-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (417963600, FixedTimespan {  // 1983-02-31T13-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (433771200, FixedTimespan {  // 1983-08-30T12-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (449586000, FixedTimespan {  // 1984-02-31T13-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (465318000, FixedTimespan {  // 1984-08-29T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (481042800, FixedTimespan {  // 1985-02-30T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (496767600, FixedTimespan {  // 1985-08-28T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (512492400, FixedTimespan {  // 1986-02-29T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (528217200, FixedTimespan {  // 1986-08-27T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (543942000, FixedTimespan {  // 1987-02-28T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (559666800, FixedTimespan {  // 1987-08-26T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (575391600, FixedTimespan {  // 1988-02-26T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (591116400, FixedTimespan {  // 1988-08-24T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (606841200, FixedTimespan {  // 1989-02-25T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (622566000, FixedTimespan {  // 1989-08-23T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (638290800, FixedTimespan {  // 1990-02-24T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (654620400, FixedTimespan {  // 1990-08-29T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (670345200, FixedTimespan {  // 1991-02-30T15-00-00 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (686073600, FixedTimespan {  // 1991-08-28T16-00-00 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (695750400, FixedTimespan {  // 1992-00-18T16-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (701784000, FixedTimespan {  // 1992-02-28T12-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (717505200, FixedTimespan {  // 1992-08-26T11-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (733244400, FixedTimespan {  // 1993-02-27T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (748969200, FixedTimespan {  // 1993-08-25T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (764694000, FixedTimespan {  // 1994-02-26T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (780418800, FixedTimespan {  // 1994-08-24T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (796143600, FixedTimespan {  // 1995-02-25T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (811868400, FixedTimespan {  // 1995-08-23T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (828198000, FixedTimespan {  // 1996-02-30T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (846342000, FixedTimespan {  // 1996-09-26T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (859647600, FixedTimespan {  // 1997-02-29T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (877791600, FixedTimespan {  // 1997-09-25T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (891097200, FixedTimespan {  // 1998-02-28T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (909241200, FixedTimespan {  // 1998-09-24T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (922546800, FixedTimespan {  // 1999-02-27T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (941295600, FixedTimespan {  // 1999-09-30T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (953996400, FixedTimespan {  // 2000-02-25T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (972745200, FixedTimespan {  // 2000-09-28T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (985446000, FixedTimespan {  // 2001-02-24T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1004194800, FixedTimespan {  // 2001-09-27T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1017500400, FixedTimespan {  // 2002-02-30T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1035644400, FixedTimespan {  // 2002-09-26T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1048950000, FixedTimespan {  // 2003-02-29T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1067094000, FixedTimespan {  // 2003-09-25T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1080399600, FixedTimespan {  // 2004-02-27T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1099148400, FixedTimespan {  // 2004-09-30T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1111849200, FixedTimespan {  // 2005-02-26T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1130598000, FixedTimespan {  // 2005-09-29T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1143298800, FixedTimespan {  // 2006-02-25T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1162047600, FixedTimespan {  // 2006-09-28T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1174748400, FixedTimespan {  // 2007-02-24T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1193497200, FixedTimespan {  // 2007-09-27T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1206802800, FixedTimespan {  // 2008-02-29T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1224946800, FixedTimespan {  // 2008-09-25T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1238252400, FixedTimespan {  // 2009-02-28T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1256396400, FixedTimespan {  // 2009-09-24T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1269702000, FixedTimespan {  // 2010-02-27T15-00-00 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   "MAGST",
        }),
        (1288450800, FixedTimespan {  // 2010-09-30T15-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1301151600, FixedTimespan {  // 2011-02-26T15-00-00 UTC
            offset: 43200,  // UTC offset 43200, DST offset 0
            is_dst: false,
            name:   "MAGT",
        }),
        (1414245600, FixedTimespan {  // 2014-09-25T14-00-00 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   "SRET",
        }),
    ]},
};


