
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Asia/Oral",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 12324,  // UTC offset 12324, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-1441164324, FixedTimespan {  // 1924-04-01T20-34-36 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (-1247544000, FixedTimespan {  // 1930-05-20T20-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (354913200, FixedTimespan {  // 1981-02-31T19-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (370720800, FixedTimespan {  // 1981-08-30T18-00-00 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (386445600, FixedTimespan {  // 1982-02-31T18-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (402256800, FixedTimespan {  // 1982-08-30T18-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (417985200, FixedTimespan {  // 1983-02-31T19-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (433792800, FixedTimespan {  // 1983-08-30T18-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (449607600, FixedTimespan {  // 1984-02-31T19-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (465339600, FixedTimespan {  // 1984-08-29T21-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (481064400, FixedTimespan {  // 1985-02-30T21-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (496789200, FixedTimespan {  // 1985-08-28T21-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (512514000, FixedTimespan {  // 1986-02-29T21-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (528238800, FixedTimespan {  // 1986-08-27T21-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (543963600, FixedTimespan {  // 1987-02-28T21-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (559688400, FixedTimespan {  // 1987-08-26T21-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (575413200, FixedTimespan {  // 1988-02-26T21-00-00 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (591138000, FixedTimespan {  // 1988-08-24T21-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (606862800, FixedTimespan {  // 1989-02-25T21-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (622591200, FixedTimespan {  // 1989-08-23T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (638316000, FixedTimespan {  // 1990-02-24T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "URAST",
        }),
        (654645600, FixedTimespan {  // 1990-08-29T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "URAT",
        }),
        (692827200, FixedTimespan {  // 1991-11-15T20-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (701809200, FixedTimespan {  // 1992-02-28T19-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (717530400, FixedTimespan {  // 1992-08-26T18-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (733269600, FixedTimespan {  // 1993-02-27T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (748994400, FixedTimespan {  // 1993-08-25T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (764719200, FixedTimespan {  // 1994-02-26T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (780444000, FixedTimespan {  // 1994-08-24T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (796168800, FixedTimespan {  // 1995-02-25T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (811893600, FixedTimespan {  // 1995-08-23T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (828223200, FixedTimespan {  // 1996-02-30T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (846367200, FixedTimespan {  // 1996-09-26T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (859672800, FixedTimespan {  // 1997-02-29T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (877816800, FixedTimespan {  // 1997-09-25T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (891122400, FixedTimespan {  // 1998-02-28T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (909266400, FixedTimespan {  // 1998-09-24T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (922572000, FixedTimespan {  // 1999-02-27T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (941320800, FixedTimespan {  // 1999-09-30T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (954021600, FixedTimespan {  // 2000-02-25T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (972770400, FixedTimespan {  // 2000-09-28T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (985471200, FixedTimespan {  // 2001-02-24T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (1004220000, FixedTimespan {  // 2001-09-27T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (1017525600, FixedTimespan {  // 2002-02-30T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (1035669600, FixedTimespan {  // 2002-09-26T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (1048975200, FixedTimespan {  // 2003-02-29T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (1067119200, FixedTimespan {  // 2003-09-25T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (1080424800, FixedTimespan {  // 2004-02-27T22-00-00 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   "ORAST",
        }),
        (1099173600, FixedTimespan {  // 2004-09-30T22-00-00 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
        (1110830400, FixedTimespan {  // 2005-02-14T20-00-00 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   "ORAT",
        }),
    ]},
};


