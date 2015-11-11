
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Pacific/Apia",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 45184,  // UTC offset 45184, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-2855737984, FixedTimespan {  // 1879-06-04T11-26-56 UTC
            offset: -37984,  // UTC offset -37984, DST offset 0
            is_dst: false,
            name:   "LMT",
        }),
        (-1861882016, FixedTimespan {  // 1911-00-01T10-33-04 UTC
            offset: -37800,  // UTC offset -37800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (-631114200, FixedTimespan {  // 1950-00-01T10-30-00 UTC
            offset: -39600,  // UTC offset -39600, DST offset 0
            is_dst: false,
            name:   "SST",
        }),
        (1285498800, FixedTimespan {  // 2010-08-26T11-00-00 UTC
            offset: -36000,  // UTC offset -39600, DST offset 3600
            is_dst: true,
            name:   "SDT",
        }),
        (1301752800, FixedTimespan {  // 2011-03-02T14-00-00 UTC
            offset: -39600,  // UTC offset -39600, DST offset 0
            is_dst: false,
            name:   "SST",
        }),
        (1316872800, FixedTimespan {  // 2011-08-24T14-00-00 UTC
            offset: -36000,  // UTC offset -39600, DST offset 3600
            is_dst: true,
            name:   "SDT",
        }),
        (1325239200, FixedTimespan {  // 2011-11-30T10-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1333202400, FixedTimespan {  // 2012-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1348927200, FixedTimespan {  // 2012-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1365256800, FixedTimespan {  // 2013-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1380376800, FixedTimespan {  // 2013-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1396706400, FixedTimespan {  // 2014-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1411826400, FixedTimespan {  // 2014-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1428156000, FixedTimespan {  // 2015-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1443276000, FixedTimespan {  // 2015-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1459605600, FixedTimespan {  // 2016-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1474725600, FixedTimespan {  // 2016-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1491055200, FixedTimespan {  // 2017-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1506175200, FixedTimespan {  // 2017-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1522504800, FixedTimespan {  // 2018-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1538229600, FixedTimespan {  // 2018-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1554559200, FixedTimespan {  // 2019-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1569679200, FixedTimespan {  // 2019-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1586008800, FixedTimespan {  // 2020-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1601128800, FixedTimespan {  // 2020-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1617458400, FixedTimespan {  // 2021-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1632578400, FixedTimespan {  // 2021-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1648908000, FixedTimespan {  // 2022-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1664028000, FixedTimespan {  // 2022-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1680357600, FixedTimespan {  // 2023-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1695477600, FixedTimespan {  // 2023-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1712412000, FixedTimespan {  // 2024-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1727532000, FixedTimespan {  // 2024-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1743861600, FixedTimespan {  // 2025-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1758981600, FixedTimespan {  // 2025-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1775311200, FixedTimespan {  // 2026-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1790431200, FixedTimespan {  // 2026-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1806760800, FixedTimespan {  // 2027-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1821880800, FixedTimespan {  // 2027-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1838210400, FixedTimespan {  // 2028-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1853330400, FixedTimespan {  // 2028-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1869660000, FixedTimespan {  // 2029-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1885384800, FixedTimespan {  // 2029-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1901714400, FixedTimespan {  // 2030-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1916834400, FixedTimespan {  // 2030-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1933164000, FixedTimespan {  // 2031-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1948284000, FixedTimespan {  // 2031-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1964613600, FixedTimespan {  // 2032-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (1979733600, FixedTimespan {  // 2032-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (1996063200, FixedTimespan {  // 2033-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2011183200, FixedTimespan {  // 2033-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2027512800, FixedTimespan {  // 2034-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2042632800, FixedTimespan {  // 2034-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2058962400, FixedTimespan {  // 2035-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2074687200, FixedTimespan {  // 2035-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2091016800, FixedTimespan {  // 2036-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2106136800, FixedTimespan {  // 2036-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2122466400, FixedTimespan {  // 2037-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2137586400, FixedTimespan {  // 2037-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2153916000, FixedTimespan {  // 2038-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2169036000, FixedTimespan {  // 2038-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2185365600, FixedTimespan {  // 2039-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2200485600, FixedTimespan {  // 2039-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2216815200, FixedTimespan {  // 2040-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2232540000, FixedTimespan {  // 2040-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2248869600, FixedTimespan {  // 2041-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2263989600, FixedTimespan {  // 2041-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2280319200, FixedTimespan {  // 2042-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2295439200, FixedTimespan {  // 2042-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2311768800, FixedTimespan {  // 2043-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2326888800, FixedTimespan {  // 2043-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2343218400, FixedTimespan {  // 2044-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2358338400, FixedTimespan {  // 2044-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2374668000, FixedTimespan {  // 2045-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2389788000, FixedTimespan {  // 2045-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2406117600, FixedTimespan {  // 2046-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2421842400, FixedTimespan {  // 2046-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2438172000, FixedTimespan {  // 2047-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2453292000, FixedTimespan {  // 2047-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2469621600, FixedTimespan {  // 2048-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2484741600, FixedTimespan {  // 2048-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2501071200, FixedTimespan {  // 2049-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2516191200, FixedTimespan {  // 2049-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2532520800, FixedTimespan {  // 2050-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2547640800, FixedTimespan {  // 2050-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2563970400, FixedTimespan {  // 2051-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2579090400, FixedTimespan {  // 2051-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2596024800, FixedTimespan {  // 2052-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2611144800, FixedTimespan {  // 2052-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2627474400, FixedTimespan {  // 2053-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2642594400, FixedTimespan {  // 2053-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2658924000, FixedTimespan {  // 2054-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2674044000, FixedTimespan {  // 2054-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2690373600, FixedTimespan {  // 2055-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2705493600, FixedTimespan {  // 2055-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2721823200, FixedTimespan {  // 2056-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2736943200, FixedTimespan {  // 2056-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2753272800, FixedTimespan {  // 2057-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2768997600, FixedTimespan {  // 2057-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2785327200, FixedTimespan {  // 2058-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2800447200, FixedTimespan {  // 2058-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2816776800, FixedTimespan {  // 2059-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2831896800, FixedTimespan {  // 2059-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2848226400, FixedTimespan {  // 2060-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2863346400, FixedTimespan {  // 2060-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2879676000, FixedTimespan {  // 2061-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2894796000, FixedTimespan {  // 2061-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2911125600, FixedTimespan {  // 2062-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2926245600, FixedTimespan {  // 2062-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2942575200, FixedTimespan {  // 2063-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2958300000, FixedTimespan {  // 2063-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (2974629600, FixedTimespan {  // 2064-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (2989749600, FixedTimespan {  // 2064-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3006079200, FixedTimespan {  // 2065-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3021199200, FixedTimespan {  // 2065-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3037528800, FixedTimespan {  // 2066-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3052648800, FixedTimespan {  // 2066-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3068978400, FixedTimespan {  // 2067-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3084098400, FixedTimespan {  // 2067-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3100428000, FixedTimespan {  // 2068-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3116152800, FixedTimespan {  // 2068-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3132482400, FixedTimespan {  // 2069-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3147602400, FixedTimespan {  // 2069-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3163932000, FixedTimespan {  // 2070-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3179052000, FixedTimespan {  // 2070-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3195381600, FixedTimespan {  // 2071-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3210501600, FixedTimespan {  // 2071-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3226831200, FixedTimespan {  // 2072-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3241951200, FixedTimespan {  // 2072-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3258280800, FixedTimespan {  // 2073-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3273400800, FixedTimespan {  // 2073-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3289730400, FixedTimespan {  // 2074-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3305455200, FixedTimespan {  // 2074-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3321784800, FixedTimespan {  // 2075-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3336904800, FixedTimespan {  // 2075-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3353234400, FixedTimespan {  // 2076-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3368354400, FixedTimespan {  // 2076-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3384684000, FixedTimespan {  // 2077-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3399804000, FixedTimespan {  // 2077-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3416133600, FixedTimespan {  // 2078-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3431253600, FixedTimespan {  // 2078-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3447583200, FixedTimespan {  // 2079-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3462703200, FixedTimespan {  // 2079-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3479637600, FixedTimespan {  // 2080-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3494757600, FixedTimespan {  // 2080-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3511087200, FixedTimespan {  // 2081-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3526207200, FixedTimespan {  // 2081-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3542536800, FixedTimespan {  // 2082-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3557656800, FixedTimespan {  // 2082-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3573986400, FixedTimespan {  // 2083-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3589106400, FixedTimespan {  // 2083-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3605436000, FixedTimespan {  // 2084-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3620556000, FixedTimespan {  // 2084-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3636885600, FixedTimespan {  // 2085-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3652610400, FixedTimespan {  // 2085-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3668940000, FixedTimespan {  // 2086-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3684060000, FixedTimespan {  // 2086-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3700389600, FixedTimespan {  // 2087-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3715509600, FixedTimespan {  // 2087-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3731839200, FixedTimespan {  // 2088-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3746959200, FixedTimespan {  // 2088-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3763288800, FixedTimespan {  // 2089-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3778408800, FixedTimespan {  // 2089-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3794738400, FixedTimespan {  // 2090-03-01T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3809858400, FixedTimespan {  // 2090-08-23T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3826188000, FixedTimespan {  // 2091-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3841912800, FixedTimespan {  // 2091-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3858242400, FixedTimespan {  // 2092-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3873362400, FixedTimespan {  // 2092-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3889692000, FixedTimespan {  // 2093-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3904812000, FixedTimespan {  // 2093-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3921141600, FixedTimespan {  // 2094-03-03T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3936261600, FixedTimespan {  // 2094-08-25T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3952591200, FixedTimespan {  // 2095-03-02T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3967711200, FixedTimespan {  // 2095-08-24T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (3984040800, FixedTimespan {  // 2096-02-31T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (3999765600, FixedTimespan {  // 2096-08-29T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (4016095200, FixedTimespan {  // 2097-03-06T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (4031215200, FixedTimespan {  // 2097-08-28T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (4047544800, FixedTimespan {  // 2098-03-05T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (4062664800, FixedTimespan {  // 2098-08-27T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
        (4078994400, FixedTimespan {  // 2099-03-04T14-00-00 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   "WSST",
        }),
        (4094114400, FixedTimespan {  // 2099-08-26T14-00-00 UTC
            offset: 50400,  // UTC offset 46800, DST offset 3600
            is_dst: true,
            name:   "WSDT",
        }),
    ]},
};


