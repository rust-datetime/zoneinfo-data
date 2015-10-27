
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Brussels",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 1050, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2450953050), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1740355200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-1693702800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-1680483600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-1663455600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-1650150000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-1632006000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-1618700400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-1613829600), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1604278800), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1585530000), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1574038800), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1552266000), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1539997200), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1520557200), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1507510800), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1490576400), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1473642000), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1459126800), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1444006800), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1427677200), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1411952400), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1396227600), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1379293200), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1364778000), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1348448400), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1333328400), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1316394000), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1301263200), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1284328800), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1269813600), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1253484000), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1238364000), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1221429600), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1206914400), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1191189600), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1175464800), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1160344800), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1143410400), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1127685600), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1111960800), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1096840800), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1080511200), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1063576800), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1049061600), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1033336800), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-1017612000), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-1002492000), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-986162400), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-969228000), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-950479200), utc_offset: 0, dst_offset: 0, name: "WET" },
        Transition { occurs_at: Some(-942012000), utc_offset: 0, dst_offset: 3600, name: "WEST" },
        Transition { occurs_at: Some(-934671600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-857257200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-844556400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-828226800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-812502000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-798073200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-781052400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-766623600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-745455600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-733273200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(228877200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(243997200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(260326800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(276051600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(291776400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(307501200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(323830800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(338950800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(354675600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(370400400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(386125200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(401850000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(417574800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(433299600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(449024400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(465354000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(481078800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(496803600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(512528400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(528253200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(543978000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(559702800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(575427600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(591152400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(606877200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(622602000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(638326800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(654656400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(670381200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(686106000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(701830800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(717555600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(733280400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(749005200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(764730000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(780454800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(796179600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(811904400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(828234000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(846378000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(859683600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(877827600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(891133200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(909277200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(922582800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(941331600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(954032400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(972781200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(985482000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1004230800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1017536400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1035680400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1048986000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1067130000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1080435600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1099184400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1111885200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1130634000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1143334800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1162083600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1174784400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1193533200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1206838800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1224982800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1238288400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1256432400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1269738000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1288486800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1301187600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1319936400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1332637200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1351386000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1364691600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1382835600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1396141200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1414285200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1427590800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1445734800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1459040400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1477789200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1490490000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1509238800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1521939600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1540688400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1553994000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1572138000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1585443600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1603587600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1616893200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1635642000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1648342800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1667091600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1679792400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1698541200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1711846800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1729990800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1743296400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1761440400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1774746000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1792890000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1806195600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1824944400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1837645200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1856394000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1869094800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1887843600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1901149200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1919293200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1932598800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1950742800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1964048400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(1982797200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(1995498000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2014246800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2026947600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2045696400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2058397200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2077146000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2090451600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2108595600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2121901200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2140045200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2153350800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2172099600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2184800400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2203549200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2216250000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2234998800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2248304400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2266448400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2279754000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2297898000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2311203600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2329347600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2342653200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2361402000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2374102800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2392851600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2405552400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2424301200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2437606800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2455750800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2469056400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2487200400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2500506000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2519254800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2531955600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2550704400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2563405200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2582154000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2595459600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2613603600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2626909200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2645053200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2658358800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2676502800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2689808400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2708557200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2721258000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2740006800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2752707600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2771456400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2784762000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2802906000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2816211600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2834355600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2847661200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2866410000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2879110800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2897859600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2910560400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2929309200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2942010000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2960758800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(2974064400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(2992208400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3005514000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3023658000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3036963600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3055712400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3068413200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3087162000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3099862800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3118611600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3131917200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3150061200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3163366800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3181510800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3194816400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3212960400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3226266000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3245014800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3257715600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3276464400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3289165200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3307914000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3321219600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3339363600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3352669200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3370813200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3384118800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3402867600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3415568400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3434317200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3447018000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3465766800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3479072400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3497216400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3510522000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3528666000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3541971600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3560115600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3573421200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3592170000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3604870800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3623619600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3636320400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3655069200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3668374800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3686518800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3699824400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3717968400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3731274000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3750022800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3762723600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3781472400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3794173200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3812922000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3825622800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3844371600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3857677200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3875821200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3889126800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3907270800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3920576400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3939325200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3952026000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(3970774800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(3983475600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(4002224400), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(4015530000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(4033674000), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(4046979600), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(4065123600), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(4078429200), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(4096573200), utc_offset: 3600, dst_offset: 0, name: "CET" },
    ],
};


