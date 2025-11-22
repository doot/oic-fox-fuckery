use icalendar::Calendar;
use loco_rs::testing::prelude::*;
use oic_fox_fuckery::app::App;
use serial_test::serial;

#[cfg(test)]
use pretty_assertions::assert_eq;

#[cfg(test)]
use mockito;

#[cfg(test)]
const TEST_EVENT_DATA: &str = r#"
{
  "_embedded": {
    "events": [
      {
        "name": "Dropkick Murphys and Bad Religion",
        "type": "event",
        "id": "G5vYZb0SsjP0H",
        "url": "https://www.ticketmaster.com/dropkick-murphys-and-bad-religion-oakland-california-07-29-2025/event/1C00624CF172593C",
        "locale": "en-us",
        "sales": { },
        "dates": {
          "start": {
            "localDate": "2025-07-29",
            "localTime": "19:00:00",
            "dateTime": "2025-07-30T02:00:00Z",
            "dateTBD": false,
            "dateTBA": false,
            "timeTBA": false,
            "noSpecificTime": false
          },
          "timezone": "America/Los_Angeles"
        }
      },
      {
        "name": "Kevin Kaarl - Ultra Sodade Tour 2025",
        "type": "event",
        "id": "G5vYZbPKOMfYi",
        "url": "https://www.ticketmaster.com/kevin-kaarl-ultra-sodade-tour-2025-oakland-california-07-30-2025/event/1C006254E0DD4C2C",
        "locale": "en-us",
        "dates": {
          "start": {
            "localDate": "2025-07-30",
            "localTime": "20:00:00",
            "dateTime": "2025-07-31T03:00:00Z",
            "dateTBD": false,
            "dateTBA": false,
            "timeTBA": false,
            "noSpecificTime": false
          },
          "timezone": "America/Los_Angeles"
        }
      },
      {
        "name": "Ben Schwartz & Friends",
        "type": "event",
        "id": "G5vYZbI6Kc0CD",
        "url": "https://www.ticketmaster.com/ben-schwartz-friends-oakland-california-08-02-2025/event/1C006281D3B643BB",
        "locale": "en-us",
        "dates": {
          "start": {
            "localDate": "2025-08-02",
            "localTime": "19:00:00",
            "dateTime": "2025-08-03T02:00:00Z",
            "dateTBD": false,
            "dateTBA": false,
            "timeTBA": false,
            "noSpecificTime": false
          },
          "timezone": "America/Los_Angeles"
        }
      },
      {
        "name": "Floating Points (LIVE) - Outside Lands Night Show",
        "type": "event",
        "id": "G5vYZbtQz60m1",
        "url": "https://www.ticketmaster.com/floating-points-live-outside-lands-night-oakland-california-08-03-2025/event/1C006296B81E4B8A",
        "locale": "en-us",
        "dates": {
          "start": {
            "localDate": "2025-08-03",
            "localTime": "19:30:00",
            "dateTime": "2025-08-04T02:30:00Z",
            "dateTBD": false,
            "dateTBA": false,
            "timeTBA": false,
            "noSpecificTime": false
          },
          "timezone": "America/Los_Angeles"
        }
      },
      {
        "name": "STRFKR",
        "type": "event",
        "id": "G5vYZbtQ_QN5d",
        "url": "https://www.ticketmaster.com/strfkr-oakland-california-08-15-2025/event/1C006296996A32C3",
        "locale": "en-us",
        "dates": {
          "start": {
            "localDate": "2025-08-15",
            "localTime": "20:00:00",
            "dateTime": "2025-08-16T03:00:00Z",
            "dateTBD": false,
            "dateTBA": false,
            "timeTBA": false,
            "noSpecificTime": false
          },
          "timezone": "America/Los_Angeles"
        }
      },
      {
        "name": "Primus - 2 DAY TICKET",
        "type": "event",
        "id": "G5vYZbs2pmKwN",
        "test": false,
        "url": "https://www.ticketmaster.com/primus-2-day-ticket-oakland-california-12-30-2025/event/1C006313D6B93CA3",
        "locale": "en-us",
        "dates": {
          "start": {
            "localDate": "2025-12-30",
            "dateTBD": false,
            "dateTBA": false,
            "timeTBA": false,
            "noSpecificTime": true
          },
          "end": {
            "localDate": "2025-12-31",
            "approximate": false,
            "noSpecificTime": true
          },
          "timezone": "America/Los_Angeles",
          "status": {
            "code": "onsale"
          },
          "spanMultipleDays": false
        }
      }
    ]
  },
  "page": {
    "size": 5,
    "totalElements": 54,
    "totalPages": 11,
    "number": 0
  }
}
"#;

#[cfg(test)]
const TEST_CAL_DATA: &str = r#"
BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//SIAHL//HockeyCalendar//EN
METHOD:PUBLISH
X-WR-CALNAME:TEAM:Sofa King Embarrassing
X-WR-RELCALID:7da15a194de3ffbe5cd554785af7a9a4
X-WR-TIMEZONE:US/Pacific
CALSCALE:GREGORIAN
LAST-MODIFIED:20250713T080124Z
BEGIN:VEVENT
UID:4851287f81cca59620e3d6723a29d698
DTSTAMP:20250713T080124Z
CATEGORIES:SIAHL
SUMMARY:Olympic Silver Bullets 2  @ Sofa King Embarrassing
LOCATION:Olympic
DESCRIPTION;ENCODING=8BIT:
 Play at Olympic =0D=0A
 SIAHL@OK Adult OK Adult OK Division 5 Silver Bullets 2 @Sofa King Embarrassing =0D=0A
DTSTART:20250730T041500Z
DTEND:20250730T053000Z
BEGIN:VALARM
UID:509df991-5d7a-4fbf-b580-82bd636c3155
DTSTAMP:20250713T080124Z
TRIGGER:-PT45M
DESCRIPTION:Event reminder
ACTION:DISPLAY
END:VALARM
END:VEVENT
BEGIN:VEVENT
UID:92a67d6f637aad82a5d966e3e805a140
DTSTAMP:20250713T080124Z
CATEGORIES:SIAHL
SUMMARY:NHL Sofa King Embarrassing  @ Oakland Reapers
LOCATION:NHL
DESCRIPTION;ENCODING=8BIT:
 Play at NHL =0D=0A
 SIAHL@OK Adult OK Adult OK Division 5 Sofa King Embarrassing @Oakland Reapers =0D=0A
DTSTART:20250612T054500Z
DTEND:20250612T070000Z
BEGIN:VALARM
UID:ae83c52f-628d-4dac-a734-0e8a02f1a37e
DTSTAMP:20250713T080124Z
TRIGGER:-PT45M
DESCRIPTION:Event reminder
ACTION:DISPLAY
END:VALARM
END:VEVENT
BEGIN:VEVENT
UID:bf5ec3807c4060ed5c2fe4f0feed58fd
DTSTAMP:20250713T080124Z
CATEGORIES:SIAHL
SUMMARY:NHL Sofa King Embarrassing  @ Lot Lizards
LOCATION:NHL
DESCRIPTION;ENCODING=8BIT:
 Play at NHL =0D=0A
 SIAHL@OK Adult OK Adult OK Division 5 Sofa King Embarrassing @Lot Lizards =0D=0A
DTSTART:20250714T044500Z
DTEND:20250714T060000Z
BEGIN:VALARM
UID:9778996b-4ce5-4aef-88a3-221485abe19b
DTSTAMP:20250713T080124Z
TRIGGER:-PT45M
DESCRIPTION:Event reminder
ACTION:DISPLAY
END:VALARM
END:VEVENT
END:VCALENDAR
"#;

#[cfg(test)]
const EXPECTED_RESPONSE: &str = r#"BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//SIAHL//HockeyCalendar//EN
METHOD:PUBLISH
X-WR-CALNAME:TEAM:Sofa King Embarrassing
X-WR-RELCALID:7da15a194de3ffbe5cd554785af7a9a4
X-WR-TIMEZONE:US/Pacific
CALSCALE:GREGORIAN
LAST-MODIFIED:20250713T080124Z
BEGIN:VEVENT
DESCRIPTION;ENCODING=8BIT:Play at Olympic =0D=0ASIAHL@OK Adult OK Adult OK 
 Division 5 Silver Bullets 2 @Sofa King Embarrassing =0D=0A
DTEND:20250730T053000Z
DTSTAMP:20250713T080124Z
DTSTART:20250730T041500Z
LOCATION:Olympic
SUMMARY:[Leave Early - Fox show at 2025-07-29 19:00:00 PDT (Dropkick Mur
 phys and Bad Religion)] Olympic Silver Bullets 2  @ Sofa King Embarrassing
UID:4851287f81cca59620e3d6723a29d698
CATEGORIES:SIAHL
BEGIN:VALARM
ACTION:DISPLAY
DESCRIPTION:Event reminder
DTSTAMP:20250713T080124Z
TRIGGER:-PT45M
UID:509df991-5d7a-4fbf-b580-82bd636c3155
END:VALARM
END:VEVENT
BEGIN:VEVENT
DESCRIPTION;ENCODING=8BIT:Play at NHL =0D=0ASIAHL@OK Adult OK Adult OK Divi
 sion 5 Sofa King Embarrassing @Oakland Reapers =0D=0A
DTEND:20250612T070000Z
DTSTAMP:20250713T080124Z
DTSTART:20250612T054500Z
LOCATION:NHL
SUMMARY:NHL Sofa King Embarrassing  @ Oakland Reapers
UID:92a67d6f637aad82a5d966e3e805a140
CATEGORIES:SIAHL
BEGIN:VALARM
ACTION:DISPLAY
DESCRIPTION:Event reminder
DTSTAMP:20250713T080124Z
TRIGGER:-PT45M
UID:ae83c52f-628d-4dac-a734-0e8a02f1a37e
END:VALARM
END:VEVENT
BEGIN:VEVENT
DESCRIPTION;ENCODING=8BIT:Play at NHL =0D=0ASIAHL@OK Adult OK Adult OK Divi
 sion 5 Sofa King Embarrassing @Lot Lizards =0D=0A
DTEND:20250714T060000Z
DTSTAMP:20250713T080124Z
DTSTART:20250714T044500Z
LOCATION:NHL
SUMMARY:NHL Sofa King Embarrassing  @ Lot Lizards
UID:bf5ec3807c4060ed5c2fe4f0feed58fd
CATEGORIES:SIAHL
BEGIN:VALARM
ACTION:DISPLAY
DESCRIPTION:Event reminder
DTSTAMP:20250713T080124Z
TRIGGER:-PT45M
UID:9778996b-4ce5-4aef-88a3-221485abe19b
END:VALARM
END:VEVENT
END:VCALENDAR
"#;

#[tokio::test]
#[serial]
async fn can_get_cal() {
    request::<App, _, _>(|request, _ctx| async move {
        let mut server = mockito::Server::new_with_opts_async(mockito::ServerOpts {
            host: "0.0.0.0",
            port: 4000,
            ..Default::default()
        })
        .await;

        let tm_mock_api = server
            .mock(
                "GET",
                "/discovery/v2/events.json?venueId=abc&size=15&sort=date,asc&apikey=test_key",
            )
            .with_body(TEST_EVENT_DATA)
            .create();
        let oic_mock_api = server
            .mock("GET", "/team-cal.php?team=123&tlev=0&tseq=0&season=456&format=iCal")
            .with_body(TEST_CAL_DATA)
            .create();

        let resp = request.get("/api/cal/123/456").await;

        // Assert that both mocked APIs were called
        tm_mock_api.assert();
        oic_mock_api.assert();

        // Ensure the response is still valid ical data
        let parsed_calendar_resp: Calendar = resp.text().parse().unwrap();
        let parsed_expected_resp: Calendar = EXPECTED_RESPONSE.parse().unwrap();

        assert_eq!(resp.status_code(), 200);
        assert_eq!(parsed_calendar_resp, parsed_expected_resp);
    })
    .await;
}
