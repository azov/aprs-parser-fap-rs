#![allow(unused_imports)] 
#![allow(dead_code)]

extern crate aprs;
extern crate fap;
#[macro_use] extern crate approx;

use aprs::{Position, Feet, Knots, KilometersPerHour,
    Meters, Degrees, Symbol, Packet as AprsPacket};

use std::borrow::Cow;


struct ParserTestCase<'a> {
   raw: &'a [u8], 
   packet_type: &'a str,
   packet_fmt: &'a str,
   src_callsign: &'a str, 
   dst_callsign: Option<&'a str>, 
   via_callsigns: Vec<&'a str>,
   via_digied: Vec<i8>,
   timestamp: Option<u64>,
   latitude: Option<f32>, 
   longitude: Option<f32>,
   ambiguity: Option<i8>,
   resolution: Option<Meters>,
   messaging: Option<i8>,
   symcode: Option<char>,
   symtable: Option<char>,
   course: Option<Degrees>,
   speed: Option<KilometersPerHour>,
   altitude: Option<Meters>,
   comment: Option<&'a str>, 
}

impl<'a> ParserTestCase<'a> {
   fn run(&self) {
      const EPSILON : f32 = 0.0001;
      let rawstr = String::from_utf8_lossy(self.raw).to_owned();
      let parsed = fap::Packet::new(self.raw).unwrap();
      let trimmed_comment = parsed.comment().map(|v| v.to_owned().trim().to_string());
      let altitude = self.altitude.map(|v| Feet::from(v).0).unwrap_or(0.0);
      let speed = self.speed.map(|v| Knots::from(v).0).unwrap_or(0.0);
      let course = self.course.map(|v| v.0).unwrap_or(0.0);
      assert_eq!( parsed.source(), self.src_callsign, "bad srccall in {:?}", rawstr);
      assert_eq!( parsed.destination(), self.dst_callsign.map(|v| Cow::Borrowed(v)), "bad dstcall in {:?}", rawstr);
      assert_eq!( trimmed_comment.as_ref().map(|v| v.as_str()), self.comment, "bad comment in {:?}", rawstr);
      assert_abs_diff_eq!( parsed.latitude().unwrap_or(0.0), self.latitude.unwrap_or(0.0), epsilon=EPSILON);
      assert_abs_diff_eq!( parsed.longitude().unwrap_or(0.0), self.longitude.unwrap_or(0.0), epsilon=EPSILON);
      assert_abs_diff_eq!( parsed.speed().map(|v| v.0).unwrap_or(0.0), speed, epsilon=EPSILON);
      assert_abs_diff_eq!( parsed.course().map(|v| v.0).unwrap_or(0.0), course, epsilon=EPSILON);
      assert_abs_diff_eq!( parsed.altitude().map(|v| v.0).unwrap_or(0.0), altitude, epsilon=EPSILON);
   }
}


#[test]
fn malformed_message() {
    let raw = "W6MTR-1>APRX28,TCPIP*,q.25N/12042.67W`APRX and Raspberry Pi powered iGate";
    assert!( fap::Packet::new(raw).is_err() );

}

// !!! CODE BELOW IS AUTOGENERATED, DO NOT EDIT !!!
#[test]
fn parse_4e5df173() {
    ParserTestCase{ 
      raw: b"VK2YCJ-9>S2U1Q2,VK2RTZ-1*,WIDE2-2,qAR,VK2ZEN-5:\x60OE p#!>/",
      packet_type: "location",
      packet_fmt: "mice",
      src_callsign: "VK2YCJ-9",
      dst_callsign: Some("S2U1Q2"), 
      via_callsigns: vec!["VK2RTZ-1", "WIDE2-2", "qAR", "VK2ZEN-5"],
      via_digied: vec![1, 0, 0, 0],
      timestamp: None,
      latitude: Some(-32.8520),
      longitude: Some(151.6840),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('>'),
      symtable: Some('/'),
      course: Some(Degrees(305.0000)),
      speed: Some(KilometersPerHour(74.0800)),
      altitude: None,
      comment: None,
   }.run();
}
    
// Py: Some("g002t055r000p000P000h76b10174eWUHU216DAVISVP2.") != Pl: Some("eWUHU216DAVISVP2.")
#[test]
fn parse_cc502c8d() {
    ParserTestCase{ 
      raw: b"CW7293>APRS,TCPXX*,qAX,CWOP-4:@080812z3718.28N/12158.90W_224/001g002t055r000p000P000h76b10174eWUHU216DAVISVP2.",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "CW7293",
      dst_callsign: Some("APRS"), 
      via_callsigns: vec!["TCPXX", "qAX", "CWOP-4"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1528445520),
      latitude: Some(37.3047),
      longitude: Some(-121.9817),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('_'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("eWUHU216DAVISVP2."),
   }.run();
}
    
// Py: Some("\x27!w@u!|3") != Pl: Some("\x27|3")
#[test]
#[ignore] // in ignored list
fn parse_a8add75e() {
    ParserTestCase{ 
      raw: b"K6CQU-3>TPTVUS,RAZOR,WIDE1*,WIDE2-1,qAS,GERLCH:\x60/\x27ll\x2293/\x27|+]$a\x27]|!w@u!|3",
      packet_type: "location",
      packet_fmt: "mice",
      src_callsign: "K6CQU-3",
      dst_callsign: Some("TPTVUS"), 
      via_callsigns: vec!["RAZOR", "WIDE1", "WIDE2-1", "qAS", "GERLCH"],
      via_digied: vec![0, 1, 0, 0, 0],
      timestamp: None,
      latitude: Some(40.7755),
      longitude: Some(-119.1967),
      ambiguity: None,
      resolution: Some(Meters(0.1852)),
      messaging: None,
      symcode: Some('3'),
      symtable: Some('/'),
      course: Some(Degrees(229.0000)),
      speed: None,
      altitude: None,
      comment: Some("\x27|3"),
   }.run();
}
    
#[test]
fn parse_9b1efae6() {
    ParserTestCase{ 
      raw: b"W7TKO-1>APMI01,TCPIP*,qAS,W7TKO:@060057z4057.57N/11742.38W#W7TKO",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "W7TKO-1",
      dst_callsign: Some("APMI01"), 
      via_callsigns: vec!["TCPIP", "qAS", "W7TKO"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1528246620),
      latitude: Some(40.9595),
      longitude: Some(-117.7063),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('#'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W7TKO"),
   }.run();
}
    
#[test]
fn parse_13ac71c0() {
    ParserTestCase{ 
      raw: b"BARBRA-9>APT311,SLIDE*,WIDE1*,WARD*,WIDE6-3,qAR,K6CDF-5:!3933.06N/11949.37W>003/011/A=004665",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "BARBRA-9",
      dst_callsign: Some("APT311"), 
      via_callsigns: vec!["SLIDE", "WIDE1", "WARD", "WIDE6-3", "qAR", "K6CDF-5"],
      via_digied: vec![1, 1, 1, 0, 0, 0],
      timestamp: None,
      latitude: Some(39.5510),
      longitude: Some(-119.8228),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('>'),
      symtable: Some('/'),
      course: Some(Degrees(3.0000)),
      speed: Some(KilometersPerHour(20.3720)),
      altitude: Some(Meters(1421.8920)),
      comment: None,
   }.run();
}
    
#[test]
fn parse_c822782a() {
    ParserTestCase{ 
      raw: b"W7TKO-1>APMI01,TCPIP*,qAS,W7TKO:@211454z4057.57N/11742.38W#W7TKO",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "W7TKO-1",
      dst_callsign: Some("APMI01"), 
      via_callsigns: vec!["TCPIP", "qAS", "W7TKO"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1526914440),
      latitude: Some(40.9595),
      longitude: Some(-117.7063),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('#'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W7TKO"),
   }.run();
}
    
#[test]
#[ignore] // in ignored list
fn parse_9bbe22d2() {
    ParserTestCase{ 
      raw: b"SNOW>APN383,qAR,KJ6IX-5:!3909.25N111952.99W#PHG2830/NVn,NCAn,TEMPn/WG6D/Snow Valley Peak, NV/A=009214",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "SNOW",
      dst_callsign: Some("APN383"), 
      via_callsigns: vec!["qAR", "KJ6IX-5"],
      via_digied: vec![0, 0],
      timestamp: None,
      latitude: Some(39.1542),
      longitude: Some(-119.8832),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('#'),
      symtable: Some('1'),
      course: None,
      speed: None,
      altitude: Some(Meters(2808.4272)),
      comment: Some("NVn,NCAn,TEMPn/WG6D/Snow Valley Peak, NV"),
   }.run();
}
    
// Py: Some("g000t054r000P000p000h37b10264 -- Red Rock/Stead, NV") != Pl: Some("-- Red Rock/Stead, NV")
#[test]
fn parse_a18be73c() {
    ParserTestCase{ 
      raw: b"KE7HLR>APAGW,TCPIP*,qAC,T2SJC:@230201z3937.66N/11954.47W_143/000g000t054r000P000p000h37b10264 -- Red Rock/Stead, NV",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "KE7HLR",
      dst_callsign: Some("APAGW"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2SJC"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1527040860),
      latitude: Some(39.6277),
      longitude: Some(-119.9078),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('_'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("-- Red Rock/Stead, NV"),
   }.run();
}
    
#[test]
fn parse_52230516() {
    ParserTestCase{ 
      raw: b"KK6IOS-9>TP5USW,qAR,W7TKO-1:\x60-MAr^Ju\x5c\x60\x22B4}_%",
      packet_type: "location",
      packet_fmt: "mice",
      src_callsign: "KK6IOS-9",
      dst_callsign: Some("TP5USW"), 
      via_callsigns: vec!["qAR", "W7TKO-1"],
      via_digied: vec![0, 0],
      timestamp: None,
      latitude: Some(40.9228),
      longitude: Some(-117.8228),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('u'),
      symtable: Some('\x5c'),
      course: Some(Degrees(246.0000)),
      speed: Some(KilometersPerHour(122.2320)),
      altitude: Some(Meters(1303.0000)),
      comment: Some("\x60_%"),
   }.run();
}
    
#[test]
fn parse_687482cc() {
    ParserTestCase{ 
      raw: b"W8KHU-9>S9SVUV,SNOW*,qAR,KJ6NKR-2:\x60/Exo\x22qj/\x60\x22Bi}_%",
      packet_type: "location",
      packet_fmt: "mice",
      src_callsign: "W8KHU-9",
      dst_callsign: Some("S9SVUV"), 
      via_callsigns: vec!["SNOW", "qAR", "KJ6NKR-2"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: Some(39.6093),
      longitude: Some(-119.6987),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('j'),
      symtable: Some('/'),
      course: Some(Degrees(285.0000)),
      speed: Some(KilometersPerHour(55.5600)),
      altitude: Some(Meters(1356.0000)),
      comment: Some("\x60_%"),
   }.run();
}
    
#[test]
#[ignore] // in ignored list
fn parse_8b7cb2d1() {
    ParserTestCase{ 
      raw: b"WINLINK>APWL2K,TCPIP*,qAS,WLNK-1:;W7DEM-10 *040129z3908.  NW11942.  Wa145.050MHz 1200 R8m Winlink Gateway",
      packet_type: "object",
      packet_fmt: "uncompressed",
      src_callsign: "WINLINK",
      dst_callsign: Some("APWL2K"), 
      via_callsigns: vec!["TCPIP", "qAS", "WLNK-1"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1528075740),
      latitude: Some(39.1417),
      longitude: Some(-119.7083),
      ambiguity: Some(2),
      resolution: Some(Meters(1852.0000)),
      messaging: None,
      symcode: Some('a'),
      symtable: Some('W'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("145.050MHz 1200 R8m Winlink Gateway"),
   }.run();
}
    
#[test]
fn parse_3e93e55b() {
    ParserTestCase{ 
      raw: b"W7TKO-1>APMI01,TCPIP*,qAS,W7TKO:@270610z4057.57N/11742.38W#W7TKO",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "W7TKO-1",
      dst_callsign: Some("APMI01"), 
      via_callsigns: vec!["TCPIP", "qAS", "W7TKO"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1527401400),
      latitude: Some(40.9595),
      longitude: Some(-117.7063),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('#'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W7TKO"),
   }.run();
}
    
#[test]
fn parse_5195699e() {
    ParserTestCase{ 
      raw: b"AG7RV-10>APDR14,TCPIP*,qAC,T2GYOR:=3932.24N/11955.59W$264/018/A=005033 https://aprsdroid.org/",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "AG7RV-10",
      dst_callsign: Some("APDR14"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2GYOR"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: Some(39.5373),
      longitude: Some(-119.9265),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('$'),
      symtable: Some('/'),
      course: Some(Degrees(264.0000)),
      speed: Some(KilometersPerHour(33.3360)),
      altitude: Some(Meters(1534.0584)),
      comment: Some("https://aprsdroid.org/"),
   }.run();
}
    
#[test]
#[ignore] // in ignored list
fn parse_6c5d81f6() {
    ParserTestCase{ 
      raw: b"WINLINK>APWL2K,TCPIP*,qAS,WLNK-1:;K6MER-10 *290825z4126.  NW12052.  Wa145.050MHz 1200 R5m Winlink Gateway",
      packet_type: "object",
      packet_fmt: "uncompressed",
      src_callsign: "WINLINK",
      dst_callsign: Some("APWL2K"), 
      via_callsigns: vec!["TCPIP", "qAS", "WLNK-1"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1527582300),
      latitude: Some(41.4417),
      longitude: Some(-120.8750),
      ambiguity: Some(2),
      resolution: Some(Meters(1852.0000)),
      messaging: None,
      symcode: Some('a'),
      symtable: Some('W'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("145.050MHz 1200 R5m Winlink Gateway"),
   }.run();
}
    
#[test]
fn parse_624e0476() {
    ParserTestCase{ 
      raw: b"AA7GT>APMI06,TCPIP*,qAC,T2NALA::AA7GT    :UNIT.Volt,Pkt,Pkt,Pcnt,None,On,On,On,On,Hi,Hi,Hi,Hi",
      packet_type: "telemetry-message",
      packet_fmt: "???",
      src_callsign: "AA7GT",
      dst_callsign: Some("APMI06"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2NALA"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: None,
      longitude: None,
      ambiguity: None,
      resolution: None,
      messaging: None,
      symcode: None,
      symtable: None,
      course: None,
      speed: None,
      altitude: None,
      comment: None,
   }.run();
}
    
#[test]
fn parse_b8b6dbfb() {
    ParserTestCase{ 
      raw: b"BOB>APU25N,SLIDE,COREY,WARD,VACA,qAR,KG7GXH-3:=/:MTJ03jE.  B",
      packet_type: "location",
      packet_fmt: "compressed",
      src_callsign: "BOB",
      dst_callsign: Some("APU25N"), 
      via_callsigns: vec!["SLIDE", "COREY", "WARD", "VACA", "qAR", "KG7GXH-3"],
      via_digied: vec![0, 0, 0, 0, 0, 0],
      timestamp: None,
      latitude: Some(39.5747),
      longitude: Some(-119.8345),
      ambiguity: None,
      resolution: Some(Meters(0.2910)),
      messaging: Some(1),
      symcode: Some('.'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: None,
   }.run();
}
    
#[test]
#[ignore] // in ignored list
fn parse_5349c755() {
    ParserTestCase{ 
      raw: b"WINLINK>APWL2K,TCPIP*,qAS,WLNK-1:;K7ET-10  *180906z3931.  NW11857.  Wa001.000MHz 1200 R5m Winlink Gateway",
      packet_type: "object",
      packet_fmt: "uncompressed",
      src_callsign: "WINLINK",
      dst_callsign: Some("APWL2K"), 
      via_callsigns: vec!["TCPIP", "qAS", "WLNK-1"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1526634360),
      latitude: Some(39.5250),
      longitude: Some(-118.9583),
      ambiguity: Some(2),
      resolution: Some(Meters(1852.0000)),
      messaging: None,
      symcode: Some('a'),
      symtable: Some('W'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("001.000MHz 1200 R5m Winlink Gateway"),
   }.run();
}
    
#[test]
fn parse_c0dbeda9() {
    ParserTestCase{ 
      raw: b"N7OCC-9>S9TRVX,VIRGPK*,WIDE1*,WIDE2-1,qAR,KG7AIQ-1:\x27.J,\x22Slk/]\x22@p}",
      packet_type: "location",
      packet_fmt: "mice",
      src_callsign: "N7OCC-9",
      dst_callsign: Some("S9TRVX"), 
      via_callsigns: vec!["VIRGPK", "WIDE1", "WIDE2-1", "qAR", "KG7AIQ-1"],
      via_digied: vec![1, 1, 0, 0, 0],
      timestamp: None,
      latitude: Some(39.7113),
      longitude: Some(-118.7693),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('k'),
      symtable: Some('/'),
      course: Some(Degrees(180.0000)),
      speed: Some(KilometersPerHour(120.3800)),
      altitude: Some(Meters(1181.0000)),
      comment: Some("]"),
   }.run();
}
    
// Py: Some("PHG7050 W2 igate Black Rock Desert K1BRC") != Pl: Some("W2 igate Black Rock Desert K1BRC")
#[test]
fn parse_3ba3219c() {
    ParserTestCase{ 
      raw: b"GERLCH>AP4R10,TCPIP*,qAC,SEVENTH:!4039.30NS11921.10W#PHG7050 W2 igate Black Rock Desert K1BRC",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "GERLCH",
      dst_callsign: Some("AP4R10"), 
      via_callsigns: vec!["TCPIP", "qAC", "SEVENTH"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: Some(40.6550),
      longitude: Some(-119.3517),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('#'),
      symtable: Some('S'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W2 igate Black Rock Desert K1BRC"),
   }.run();
}
    
#[test]
#[ignore] // in ignored list
fn parse_73dab7a9() {
    ParserTestCase{ 
      raw: b"HOUGH>APNKMP,KE6REA-3*,WIDE2-1,qAR,K6TTR-6:!4002.68NS12053.16W#PHG2134/NCAn/HOUGH/A=007240",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "HOUGH",
      dst_callsign: Some("APNKMP"), 
      via_callsigns: vec!["KE6REA-3", "WIDE2-1", "qAR", "K6TTR-6"],
      via_digied: vec![1, 0, 0, 0],
      timestamp: None,
      latitude: Some(40.0447),
      longitude: Some(-120.8860),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('#'),
      symtable: Some('S'),
      course: None,
      speed: None,
      altitude: Some(Meters(2206.7520)),
      comment: Some("NCAn/HOUGH"),
   }.run();
}
    
#[test]
fn parse_f7a710ac() {
    ParserTestCase{ 
      raw: b"AA7GT>APMI06,TCPIP*,qAC,T2USANE::AA7GT    :PARM.Vin,Rx1h,Dg1h,Eff1h,A5,O1,O2,O3,O4,I1,I2,I3,I4",
      packet_type: "telemetry-message",
      packet_fmt: "???",
      src_callsign: "AA7GT",
      dst_callsign: Some("APMI06"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2USANE"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: None,
      longitude: None,
      ambiguity: None,
      resolution: None,
      messaging: None,
      symcode: None,
      symtable: None,
      course: None,
      speed: None,
      altitude: None,
      comment: None,
   }.run();
}
    
#[test]
fn parse_b7b26e8c() {
    ParserTestCase{ 
      raw: b"AA7GT>APMI06,TCPIP*,qAC,T2CAEAST::AA7GT    :UNIT.Volt,Pkt,Pkt,Pcnt,None,On,On,On,On,Hi,Hi,Hi,Hi",
      packet_type: "telemetry-message",
      packet_fmt: "???",
      src_callsign: "AA7GT",
      dst_callsign: Some("APMI06"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2CAEAST"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: None,
      longitude: None,
      ambiguity: None,
      resolution: None,
      messaging: None,
      symcode: None,
      symtable: None,
      course: None,
      speed: None,
      altitude: None,
      comment: None,
   }.run();
}
    
#[test]
#[ignore] // in ignored list
fn parse_e57cfcb2() {
    ParserTestCase{ 
      raw: b"VIRGPK>APNU19,qAR,KE7UQK-3:!3945.37NS11927.68W#PHG7700/W3,NVn, WA6TLW, Virginia Pk A=008367",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "VIRGPK",
      dst_callsign: Some("APNU19"), 
      via_callsigns: vec!["qAR", "KE7UQK-3"],
      via_digied: vec![0, 0],
      timestamp: None,
      latitude: Some(39.7562),
      longitude: Some(-119.4613),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('#'),
      symtable: Some('S'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W3,NVn, WA6TLW, Virginia Pk A=008367"),
   }.run();
}
    
#[test]
fn parse_cb82e2a8() {
    ParserTestCase{ 
      raw: b"AA7GT>APMI06,TCPIP*,qAC,T2CAEAST:@251722z3938.10N/12013.33W-WX3in1Plus2.0 U=12.1V,T=0.0C",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "AA7GT",
      dst_callsign: Some("APMI06"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2CAEAST"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1527268920),
      latitude: Some(39.6350),
      longitude: Some(-120.2222),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('-'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("WX3in1Plus2.0 U=12.1V,T=0.0C"),
   }.run();
}
    
#[test]
fn parse_643d8bff() {
    ParserTestCase{ 
      raw: b"KE7UQK-3>APDW13,TCPIP*,qAC,T2ONTARIO:!3939.73NR11953.14W&Direwolf 1.3 ON RPi+SDR",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "KE7UQK-3",
      dst_callsign: Some("APDW13"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2ONTARIO"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: Some(39.6622),
      longitude: Some(-119.8857),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('&'),
      symtable: Some('R'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("Direwolf 1.3 ON RPi+SDR"),
   }.run();
}
    
#[test]
fn parse_47243059() {
    ParserTestCase{ 
      raw: b"KG7GXH-3>APDW13,TCPIP*,qAC,T2CAWEST:!3927.64NR11949.28W&Direwolf 1.3 ON RPi+SDR",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "KG7GXH-3",
      dst_callsign: Some("APDW13"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2CAWEST"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: Some(39.4607),
      longitude: Some(-119.8213),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('&'),
      symtable: Some('R'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("Direwolf 1.3 ON RPi+SDR"),
   }.run();
}
    
#[test]
fn parse_ff253b6d() {
    ParserTestCase{ 
      raw: b"W7TKO-1>APMI01,TCPIP*,qAS,W7TKO:@021403z4057.57N/11742.38W#W7TKO",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "W7TKO-1",
      dst_callsign: Some("APMI01"), 
      via_callsigns: vec!["TCPIP", "qAS", "W7TKO"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1527948180),
      latitude: Some(40.9595),
      longitude: Some(-117.7063),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('#'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W7TKO"),
   }.run();
}
    
#[test]
#[ignore] // in ignored list
fn parse_2caaa37c() {
    ParserTestCase{ 
      raw: b"VIRGPK>APNU19,qAS,GERLCH:!3945.37NS11927.68W#PHG7700/W3,NVn, WA6TLW, Virginia Pk A=008367",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "VIRGPK",
      dst_callsign: Some("APNU19"), 
      via_callsigns: vec!["qAS", "GERLCH"],
      via_digied: vec![0, 0],
      timestamp: None,
      latitude: Some(39.7562),
      longitude: Some(-119.4613),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('#'),
      symtable: Some('S'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W3,NVn, WA6TLW, Virginia Pk A=008367"),
   }.run();
}
    
// Py: Some("g003t039r001p001P001h54b10246L174.DsVP") != Pl: Some(".DsVP")
#[test]
fn parse_671c53b1() {
    ParserTestCase{ 
      raw: b"K6TTR>APRS,TCPIP*,qAC,T2BC:@051953z4017.63N/12103.17W_016/000g003t039r001p001P001h54b10246L174.DsVP",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "K6TTR",
      dst_callsign: Some("APRS"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2BC"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1528228380),
      latitude: Some(40.2938),
      longitude: Some(-121.0528),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('_'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some(".DsVP"),
   }.run();
}
    
#[test]
fn parse_5a7740b9() {
    ParserTestCase{ 
      raw: b"KG7BUN-2>TR1RTT,KE7QP,WIDE1,KFALLS,WIDE2*,qAR,ASHLND:\x6005\x1fl \x1ck/\x60\x22Cj}_\x22",
      packet_type: "location",
      packet_fmt: "mice",
      src_callsign: "KG7BUN-2",
      dst_callsign: Some("TR1RTT"), 
      via_callsigns: vec!["KE7QP", "WIDE1", "KFALLS", "WIDE2", "qAR", "ASHLND"],
      via_digied: vec![0, 0, 0, 1, 0, 0],
      timestamp: None,
      latitude: Some(42.2073),
      longitude: Some(-120.4172),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('k'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: Some(Meters(1448.0000)),
      comment: Some("\x60_\x22"),
   }.run();
}
    
#[test]
fn parse_c4da2269() {
    ParserTestCase{ 
      raw: b"W7TKO-1>APMI01,TCPIP*,qAS,W7TKO:@120831z4057.57N/11742.38W#W7TKO",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "W7TKO-1",
      dst_callsign: Some("APMI01"), 
      via_callsigns: vec!["TCPIP", "qAS", "W7TKO"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1526113860),
      latitude: Some(40.9595),
      longitude: Some(-117.7063),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('#'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W7TKO"),
   }.run();
}
    
#[test]
#[ignore] // in ignored list
fn parse_e44a7b1a() {
    ParserTestCase{ 
      raw: b"KE6REA-3>APN382,qAR,K6TTR-6:!4020.22NS12052.23W#PHG5810/W2,NCAn/HAMILTON RIDGE/A=006850",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "KE6REA-3",
      dst_callsign: Some("APN382"), 
      via_callsigns: vec!["qAR", "K6TTR-6"],
      via_digied: vec![0, 0],
      timestamp: None,
      latitude: Some(40.3370),
      longitude: Some(-120.8705),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('#'),
      symtable: Some('S'),
      course: None,
      speed: None,
      altitude: Some(Meters(2087.8800)),
      comment: Some("W2,NCAn/HAMILTON RIDGE"),
   }.run();
}
    
#[test]
fn parse_56973c29() {
    ParserTestCase{ 
      raw: b"AA7GT>APMI06,TCPIP*,qAC,T2MSSOURI::AA7GT    :PARM.Vin,Rx1h,Dg1h,Eff1h,A5,O1,O2,O3,O4,I1,I2,I3,I4",
      packet_type: "telemetry-message",
      packet_fmt: "???",
      src_callsign: "AA7GT",
      dst_callsign: Some("APMI06"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2MSSOURI"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: None,
      longitude: None,
      ambiguity: None,
      resolution: None,
      messaging: None,
      symcode: None,
      symtable: None,
      course: None,
      speed: None,
      altitude: None,
      comment: None,
   }.run();
}
    
#[test]
fn parse_70bece3f() {
    ParserTestCase{ 
      raw: b"AA7GT>APMI06,TCPIP*,qAC,T2EDM::AA7GT    :UNIT.Volt,Pkt,Pkt,Pcnt,None,On,On,On,On,Hi,Hi,Hi,Hi",
      packet_type: "telemetry-message",
      packet_fmt: "???",
      src_callsign: "AA7GT",
      dst_callsign: Some("APMI06"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2EDM"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: None,
      longitude: None,
      ambiguity: None,
      resolution: None,
      messaging: None,
      symcode: None,
      symtable: None,
      course: None,
      speed: None,
      altitude: None,
      comment: None,
   }.run();
}
    
#[test]
fn parse_8547b698() {
    ParserTestCase{ 
      raw: b"AA7GT>APMI06,TCPIP*,qAC,T2PR:@040134z3938.10N/12013.33W-WX3in1Plus2.0 U=12.1V,T=0.0C",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "AA7GT",
      dst_callsign: Some("APMI06"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2PR"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1528076040),
      latitude: Some(39.6350),
      longitude: Some(-120.2222),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('-'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("WX3in1Plus2.0 U=12.1V,T=0.0C"),
   }.run();
}
    
#[test]
fn parse_42099a44() {
    ParserTestCase{ 
      raw: b"W7TKO-1>APMI01,TCPIP*,qAS,W7TKO:@040141z4057.57N/11742.38W#W7TKO",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "W7TKO-1",
      dst_callsign: Some("APMI01"), 
      via_callsigns: vec!["TCPIP", "qAS", "W7TKO"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1528076460),
      latitude: Some(40.9595),
      longitude: Some(-117.7063),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('#'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W7TKO"),
   }.run();
}
    
#[test]
#[ignore] // in ignored list
fn parse_f17c8963() {
    ParserTestCase{ 
      raw: b"WINLINK>APWL2K,TCPIP*,qAS,WLNK-1:;K7ET-10  *090632z3931.  NW11857.  Wa001.000MHz 1200 R5m Winlink Gateway",
      packet_type: "object",
      packet_fmt: "uncompressed",
      src_callsign: "WINLINK",
      dst_callsign: Some("APWL2K"), 
      via_callsigns: vec!["TCPIP", "qAS", "WLNK-1"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1528525920),
      latitude: Some(39.5250),
      longitude: Some(-118.9583),
      ambiguity: Some(2),
      resolution: Some(Meters(1852.0000)),
      messaging: None,
      symcode: Some('a'),
      symtable: Some('W'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("001.000MHz 1200 R5m Winlink Gateway"),
   }.run();
}
    
#[test]
fn parse_f1d0161a() {
    ParserTestCase{ 
      raw: b"AA7GT>APMI06,TCPIP*,qAC,T2PR:@121151z3938.10N/12013.33W-WX3in1Plus2.0 U=12.1V,T=0.0C",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "AA7GT",
      dst_callsign: Some("APMI06"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2PR"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1526125860),
      latitude: Some(39.6350),
      longitude: Some(-120.2222),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('-'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("WX3in1Plus2.0 U=12.1V,T=0.0C"),
   }.run();
}
    
// Py: Some("g009t036r000p...P000h51b10235L000.DsVP") != Pl: None
#[test]
fn parse_2ccc962b() {
    ParserTestCase{ 
      raw: b"WA7ITP>APRS,TCPIP*,qAC,THIRD:@130443z3925.85N/11948.27W_309/005g009t036r000p...P000h51b10235L000.DsVP",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "WA7ITP",
      dst_callsign: Some("APRS"), 
      via_callsigns: vec!["TCPIP", "qAC", "THIRD"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1526186580),
      latitude: Some(39.4308),
      longitude: Some(-119.8045),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('_'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: None,
   }.run();
}
    
// Py: Some("PHG7050 W2 Black Rock Desert K1BRC") != Pl: Some("W2 Black Rock Desert K1BRC")
#[test]
fn parse_428ae3a3() {
    ParserTestCase{ 
      raw: b"GERLCH>AP4R10,qAR,RAZOR:!4039.30NS11921.10W#PHG7050 W2 Black Rock Desert K1BRC",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "GERLCH",
      dst_callsign: Some("AP4R10"), 
      via_callsigns: vec!["qAR", "RAZOR"],
      via_digied: vec![0, 0],
      timestamp: None,
      latitude: Some(40.6550),
      longitude: Some(-119.3517),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('#'),
      symtable: Some('S'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("W2 Black Rock Desert K1BRC"),
   }.run();
}
    
#[test]
fn parse_26029382() {
    ParserTestCase{ 
      raw: b"KG7PDC>APBM1S,TCPIP*,qAS,N6BMW-15:@302008z3953.70N/11957.48W-openSPOT 441.0125/441.0125 CC1",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "KG7PDC",
      dst_callsign: Some("APBM1S"), 
      via_callsigns: vec!["TCPIP", "qAS", "N6BMW-15"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1527710880),
      latitude: Some(39.8950),
      longitude: Some(-119.9580),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: Some(1),
      symcode: Some('-'),
      symtable: Some('/'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("openSPOT 441.0125/441.0125 CC1"),
   }.run();
}
    
#[test]
fn parse_25bb8dc2() {
    ParserTestCase{ 
      raw: b"AA7GT>APMI06,TCPIP*,qAC,T2VAN::AA7GT    :EQNS.0,0.079,0,0,10,0,0,10,0,0,1,0,0,0,0",
      packet_type: "telemetry-message",
      packet_fmt: "???",
      src_callsign: "AA7GT",
      dst_callsign: Some("APMI06"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2VAN"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: None,
      longitude: None,
      ambiguity: None,
      resolution: None,
      messaging: None,
      symcode: None,
      symtable: None,
      course: None,
      speed: None,
      altitude: None,
      comment: None,
   }.run();
}
    
// Py: Some("\x27!w[a!|3") != Pl: Some("\x27|3")
#[test]
#[ignore] // in ignored list
fn parse_a4086e3d() {
    ParserTestCase{ 
      raw: b"NN7K-9>SY3STX,SNOW*,WIDE2-2,qAR,KJ6NKR-2:\x60/Fwl\x22\x1eV/\x27\x22BM}|)U%L\x27n|!w[a!|3",
      packet_type: "location",
      packet_fmt: "mice",
      src_callsign: "NN7K-9",
      dst_callsign: Some("SY3STX"), 
      via_callsigns: vec!["SNOW", "WIDE2-2", "qAR", "KJ6NKR-2"],
      via_digied: vec![1, 0, 0, 0],
      timestamp: None,
      latitude: Some(39.5580),
      longitude: Some(-119.7152),
      ambiguity: None,
      resolution: Some(Meters(0.1852)),
      messaging: None,
      symcode: Some('V'),
      symtable: Some('/'),
      course: Some(Degrees(202.0000)),
      speed: None,
      altitude: Some(Meters(1328.0000)),
      comment: Some("\x27|3"),
   }.run();
}
    
#[test]
fn parse_1db72122() {
    ParserTestCase{ 
      raw: b"BOB>APU25N,TCPIP*,qAC,T2SJC:>081649z==>   WA6TLW",
      packet_type: "status",
      packet_fmt: "???",
      src_callsign: "BOB",
      dst_callsign: Some("APU25N"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2SJC"],
      via_digied: vec![1, 0, 0],
      timestamp: Some(1528476540),
      latitude: None,
      longitude: None,
      ambiguity: None,
      resolution: None,
      messaging: None,
      symcode: None,
      symtable: None,
      course: None,
      speed: None,
      altitude: None,
      comment: None,
   }.run();
}
    
// Py: Some("RNG0001 440 Voice 446.50000MHz +0.0000MHz") != Pl: Some("440 Voice 446.50000MHz +0.0000MHz")
#[test]
fn parse_03e70113() {
    ParserTestCase{ 
      raw: b"AE7JW-B>APDG02,TCPIP*,qAC,AE7JW-BS:!3929.40ND11951.00W&RNG0001 440 Voice 446.50000MHz +0.0000MHz",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "AE7JW-B",
      dst_callsign: Some("APDG02"), 
      via_callsigns: vec!["TCPIP", "qAC", "AE7JW-BS"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: Some(39.4900),
      longitude: Some(-119.8500),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('&'),
      symtable: Some('D'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("440 Voice 446.50000MHz +0.0000MHz"),
   }.run();
}
    
#[test]
fn parse_18900633() {
    ParserTestCase{ 
      raw: b"KE7UQK-2>APRX28,TCPIP*,qAC,T2NALA:!3939.73NR11953.13W&Rx-iGate in-a-box | aprx 2.08.593g on RPi",
      packet_type: "location",
      packet_fmt: "uncompressed",
      src_callsign: "KE7UQK-2",
      dst_callsign: Some("APRX28"), 
      via_callsigns: vec!["TCPIP", "qAC", "T2NALA"],
      via_digied: vec![1, 0, 0],
      timestamp: None,
      latitude: Some(39.6622),
      longitude: Some(-119.8855),
      ambiguity: None,
      resolution: Some(Meters(18.5200)),
      messaging: None,
      symcode: Some('&'),
      symtable: Some('R'),
      course: None,
      speed: None,
      altitude: None,
      comment: Some("Rx-iGate in-a-box | aprx 2.08.593g on RPi"),
   }.run();
}
    
// Processed        45 line(s)
// Parsed:          46 
// Has location:    39