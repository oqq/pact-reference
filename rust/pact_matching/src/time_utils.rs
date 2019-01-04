use nom::types::CompleteStr;
use nom::digit1;
use itertools::Itertools;

//a	Am/pm marker	Text	PM
//H	Hour in day (0-23)	Number	0
//k	Hour in day (1-24)	Number	24
//K	Hour in am/pm (0-11)	Number	0
//h	Hour in am/pm (1-12)	Number	12
//m	Minute in hour	Number	30
//s	Second in minute	Number	55
//S	Millisecond	Number	978
//z	Time zone	General time zone	Pacific Standard Time; PST; GMT-08:00
//Z	Time zone	RFC 822 time zone	-0800
//X	Time zone	ISO 8601 time zone	-08; -0800; -08:00

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimePatternToken {
  Era,
  Year,
  Month,
  Text(Vec<char>),
  WeekInYear,
  WeekInMonth,
  DayInYear,
  DayInMonth,
  DayOfWeekInMonth,
  DayName,
  DayOfWeek
}

fn is_digit(ch: char) -> bool {
  ch.is_ascii_digit()
}

fn validate_number(m: CompleteStr, num_type: String, lower: usize, upper: usize) -> Result<CompleteStr, String> {
  match m.0.parse::<usize>() {
    Ok(v) => if v >= lower && v <= upper {
      Ok(m)
    } else {
      Err(format!("Invalid {} {}", num_type, v))
    },
    Err(err) => Err(format!("{}", err))
  }
}

fn validate_month(m: CompleteStr) -> Result<CompleteStr, String> {
  validate_number(m, "month".into(), 1, 12)
}

fn validate_week_in_year(m: CompleteStr) -> Result<CompleteStr, String> {
  validate_number(m, "week in year".into(), 1, 56)
}

fn validate_week_in_month(m: CompleteStr) -> Result<CompleteStr, String> {
  validate_number(m, "week in month".into(), 1, 5)
}

fn validate_day_in_year(m: CompleteStr) -> Result<CompleteStr, String> {
  validate_number(m, "day in year".into(), 1, 356)
}

fn validate_day_in_month(m: CompleteStr) -> Result<CompleteStr, String> {
  validate_number(m, "day in month".into(), 1, 31)
}

fn validate_day_of_week(m: CompleteStr) -> Result<CompleteStr, String> {
  validate_number(m, "day of week".into(), 1, 7)
}

named!(era_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::Era, many1!(char!('G'))));
named!(week_in_year_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::WeekInYear, many1!(char!('w'))));
named!(week_in_month_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::WeekInMonth, many1!(char!('W'))));
named!(day_in_year_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::DayInYear, many1!(char!('D'))));
named!(day_in_month_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::DayInMonth, many1!(char!('d'))));
named!(day_of_week_in_month_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::DayOfWeekInMonth, many1!(char!('F'))));
named!(day_name_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::DayName, many1!(char!('E'))));
named!(day_of_week_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::DayOfWeek, many1!(char!('u'))));
named!(year_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::Year, many1!(is_a!("yY"))));
named!(month_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::Month, many1!(is_a!("ML"))));
named!(text_pattern <CompleteStr, DateTimePatternToken>, do_parse!(
  t: many1!(none_of!("GyYMLwWdDFEu'"))
  >> (DateTimePatternToken::Text(t))
));
named!(quoted_text_pattern <CompleteStr, DateTimePatternToken>, do_parse!(
  char!('\'')
  >> t: many1!(alt!(tag!("''") | is_not!("'")))
  >> char!('\'')
  >> (DateTimePatternToken::Text(t.iter()
    .map(|s| s.chars().coalesce(|x, y| if x == '\'' && y == '\'' { Ok('\'') } else { Err((x, y)) }).collect::<String>())
    .join("").chars().collect()))
));
named!(quote_pattern <CompleteStr, DateTimePatternToken>, value!(DateTimePatternToken::Text("'".chars().collect()), tag!("''")));
named!(parse_pattern <CompleteStr, Vec<DateTimePatternToken> >, do_parse!(
  v: many0!(alt!(
    era_pattern |
    year_pattern |
    month_pattern |
    week_in_year_pattern |
    week_in_month_pattern |
    day_in_year_pattern |
    day_in_month_pattern |
    day_of_week_in_month_pattern |
    day_name_pattern |
    day_of_week_pattern |
    quoted_text_pattern |
    quote_pattern |
    text_pattern)) >> (v)
));

named!(era <CompleteStr, CompleteStr>, alt!(tag_no_case!("ad") | tag_no_case!("bc")));
named!(month_text <CompleteStr, CompleteStr>, alt!(
  tag_no_case!("january")   | tag_no_case!("jan") |
  tag_no_case!("february")  | tag_no_case!("feb") |
  tag_no_case!("march")     | tag_no_case!("mar") |
  tag_no_case!("april")     | tag_no_case!("apr") |
  tag_no_case!("may")       | tag_no_case!("may") |
  tag_no_case!("june")      | tag_no_case!("jun") |
  tag_no_case!("july")      | tag_no_case!("jul") |
  tag_no_case!("august")    | tag_no_case!("aug") |
  tag_no_case!("september") | tag_no_case!("sep") |
  tag_no_case!("october")   | tag_no_case!("oct") |
  tag_no_case!("november")  | tag_no_case!("nov") |
  tag_no_case!("december")  | tag_no_case!("dec")
));
named!(month_num <CompleteStr, CompleteStr>, map_res!(take_while_m_n!(1, 2, is_digit), validate_month));
named!(month <CompleteStr, CompleteStr>, alt!(month_text | month_num));
named!(week_in_year <CompleteStr, CompleteStr>, map_res!(take_while_m_n!(1, 2, is_digit), validate_week_in_year));
named!(week_in_month <CompleteStr, CompleteStr>, map_res!(take_while_m_n!(1, 2, is_digit), validate_week_in_month));
named!(day_in_year <CompleteStr, CompleteStr>, map_res!(take_while_m_n!(1, 2, is_digit), validate_day_in_year));
named!(day_in_month <CompleteStr, CompleteStr>, map_res!(take_while_m_n!(1, 2, is_digit), validate_day_in_month));
named!(day_of_week <CompleteStr, CompleteStr>, map_res!(take_while_m_n!(1, 1, is_digit), validate_day_of_week));
named_args!(text<'a>(t: &'a Vec<char>) <CompleteStr<'a>, CompleteStr<'a>>, tag!(t.iter().collect::<String>().as_str()));
named!(day_of_week_name <CompleteStr, CompleteStr>, alt!(
  tag_no_case!("sunday")    | tag_no_case!("sun") |
  tag_no_case!("monday")    | tag_no_case!("mon") |
  tag_no_case!("tuesday")   | tag_no_case!("tue") |
  tag_no_case!("wednesday") | tag_no_case!("wed") |
  tag_no_case!("thursday")  | tag_no_case!("thu") |
  tag_no_case!("friday")    | tag_no_case!("fri") |
  tag_no_case!("saturday")  | tag_no_case!("sat")
));

fn validate_datetime_string<'a>(value: &String, pattern_tokens: &Vec<DateTimePatternToken>) -> Result<(), String> {
  p!(value);
  p!(pattern_tokens);
  let mut buffer = CompleteStr(&value);
  for token in pattern_tokens {
    let result = match token {
      DateTimePatternToken::Era => era(buffer),
      DateTimePatternToken::Year => digit1(buffer),
      DateTimePatternToken::WeekInYear => week_in_year(buffer),
      DateTimePatternToken::WeekInMonth => week_in_month(buffer),
      DateTimePatternToken::DayInYear => day_in_year(buffer),
      DateTimePatternToken::DayInMonth => day_in_month(buffer),
      DateTimePatternToken::Month => month(buffer),
      DateTimePatternToken::Text(t) => text(buffer, t),
      DateTimePatternToken::DayOfWeekInMonth => digit1(buffer),
      DateTimePatternToken::DayName => day_of_week_name(buffer),
      DateTimePatternToken::DayOfWeek => day_of_week(buffer)
    }.map_err(|err| format!("{:?}", err))?;
    buffer = result.0;
  }

  if buffer.len() > 0 {
    Err(format!("Remaining data after applying pattern {:?}", buffer))
  } else {
    Ok(())
  }
}

pub fn validate_datetime(value: &String, format: &String) -> Result<(), String> {
  match parse_pattern(CompleteStr(format.as_str())) {
    Ok(pattern_tokens) => validate_datetime_string(value, &pattern_tokens.1),
    Err(err) => Err(format!("{:?}", err))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use expectest::prelude::*;


  #[test]
  fn parse_date_and_time() {
    expect!(validate_datetime(&"2001-01-02".into(), &"yyyy-MM-dd".into())).to(be_ok());
    expect!(validate_datetime(&"2001-01-02 12:33:45".into(), &"yyyy-MM-dd HH:mm:ss".into())).to(be_ok());

//    "yyyy.MM.dd G 'at' HH:mm:ss z"	2001.07.04 AD at 12:08:56 PDT
    expect!(validate_datetime(&"Wed, Jul 4, '01".into(), &"EEE, MMM d, ''yy".into())).to(be_ok());

//    "h:mm a"	12:08 PM
//    "hh 'o''clock' a, zzzz"	12 o'clock PM, Pacific Daylight Time
//    "K:mm a, z"	0:08 PM, PDT
//    "yyyyy.MMMMM.dd GGG hh:mm aaa"	02001.July.04 AD 12:08 PM
//    "EEE, d MMM yyyy HH:mm:ss Z"	Wed, 4 Jul 2001 12:08:56 -0700
//    "yyMMddHHmmssZ"	010704120856-0700
//    "yyyy-MM-dd'T'HH:mm:ss.SSSZ"	2001-07-04T12:08:56.235-0700
//    "yyyy-MM-dd'T'HH:mm:ss.SSSXXX"	2001-07-04T12:08:56.235-07:00

    expect!(validate_datetime(&"2001-W27-3".into(), &"YYYY-'W'ww-u".into())).to(be_ok());
  }

  #[test]
  fn parse_era() {
    expect!(parse_pattern(CompleteStr("G"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Era])));
    expect!(parse_pattern(CompleteStr("GG"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Era])));
    expect!(parse_pattern(CompleteStr("GGGGG"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Era])));

    expect!(validate_datetime(&"ad".into(), &"G".into())).to(be_ok());
    expect!(validate_datetime(&"AD".into(), &"GG".into())).to(be_ok());
    expect!(validate_datetime(&"bc".into(), &"GGG".into())).to(be_ok());
    expect!(validate_datetime(&"BC".into(), &"G".into())).to(be_ok());
    expect!(validate_datetime(&"BX".into(), &"G".into())).to(be_err());
  }

  #[test]
  fn parse_year() {
    expect!(parse_pattern(CompleteStr("y"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Year])));
    expect!(parse_pattern(CompleteStr("yy"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Year])));
    expect!(parse_pattern(CompleteStr("yyyy"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Year])));
    expect!(parse_pattern(CompleteStr("YYyy"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Year])));

    expect!(validate_datetime(&"2000".into(), &"y".into())).to(be_ok());
    expect!(validate_datetime(&"2000".into(), &"yy".into())).to(be_ok());
    expect!(validate_datetime(&"2000".into(), &"YYYY".into())).to(be_ok());
    expect!(validate_datetime(&"20".into(), &"yy".into())).to(be_ok());
    expect!(validate_datetime(&"20".into(), &"yyyy".into())).to(be_ok());
    expect!(validate_datetime(&"".into(), &"yyyy".into())).to(be_err());
  }

  #[test]
  fn parse_month() {
    expect!(parse_pattern(CompleteStr("M"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Month])));
    expect!(parse_pattern(CompleteStr("MM"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Month])));
    expect!(parse_pattern(CompleteStr("LLL"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Month])));

    expect!(validate_datetime(&"jan".into(), &"M".into())).to(be_ok());
    expect!(validate_datetime(&"october".into(), &"MMM".into())).to(be_ok());
    expect!(validate_datetime(&"December".into(), &"L".into())).to(be_ok());
    expect!(validate_datetime(&"01".into(), &"L".into())).to(be_ok());
    expect!(validate_datetime(&"10".into(), &"MM".into())).to(be_ok());
    expect!(validate_datetime(&"100".into(), &"MM".into())).to(be_err());
    expect!(validate_datetime(&"13".into(), &"MM".into())).to(be_err());
    expect!(validate_datetime(&"31".into(), &"MM".into())).to(be_err());
    expect!(validate_datetime(&"00".into(), &"MM".into())).to(be_err());
    expect!(validate_datetime(&"".into(), &"MMM".into())).to(be_err());
  }

  #[test]
  fn parse_text() {
    expect!(parse_pattern(CompleteStr("ello"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Text("ello".chars().collect())])));
    expect!(parse_pattern(CompleteStr("'dd-MM-yyyy'"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Text("dd-MM-yyyy".chars().collect())])));
    expect!(parse_pattern(CompleteStr("''"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Text("'".chars().collect())])));
    expect!(parse_pattern(CompleteStr("'dd-''MM''-yyyy'"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::Text("dd-'MM'-yyyy".chars().collect())])));

    expect!(validate_datetime(&"ello".into(), &"ello".into())).to(be_ok());
    expect!(validate_datetime(&"elo".into(), &"ello".into())).to(be_err());
    expect!(validate_datetime(&"dd-MM-yyyy".into(), &"'dd-MM-yyyy'".into())).to(be_ok());
  }

  #[test]
  fn parse_week_number() {
    expect!(parse_pattern(CompleteStr("wW"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::WeekInYear, DateTimePatternToken::WeekInMonth])));
    expect!(parse_pattern(CompleteStr("www"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::WeekInYear])));
    expect!(parse_pattern(CompleteStr("WW"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::WeekInMonth])));

    expect!(validate_datetime(&"12".into(), &"w".into())).to(be_ok());
    expect!(validate_datetime(&"3".into(), &"WW".into())).to(be_ok());
    expect!(validate_datetime(&"57".into(), &"ww".into())).to(be_err());
    expect!(validate_datetime(&"0".into(), &"W".into())).to(be_err());
  }

  #[test]
  fn parse_day_number() {
    expect!(parse_pattern(CompleteStr("dD"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::DayInMonth, DateTimePatternToken::DayInYear])));
    expect!(parse_pattern(CompleteStr("dd"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::DayInMonth])));
    expect!(parse_pattern(CompleteStr("DDD"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::DayInYear])));

    expect!(validate_datetime(&"12".into(), &"d".into())).to(be_ok());
    expect!(validate_datetime(&"03".into(), &"DD".into())).to(be_ok());
    expect!(validate_datetime(&"32".into(), &"dd".into())).to(be_err());
    expect!(validate_datetime(&"0".into(), &"D".into())).to(be_err());
  }

  #[test]
  fn parse_day_of_week() {
    expect!(parse_pattern(CompleteStr("F"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::DayOfWeekInMonth])));
    expect!(parse_pattern(CompleteStr("EE"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::DayName])));
    expect!(parse_pattern(CompleteStr("u"))).to(
      be_ok().value((CompleteStr(""), vec![DateTimePatternToken::DayOfWeek])));

    expect!(validate_datetime(&"12".into(), &"F".into())).to(be_ok());
    expect!(validate_datetime(&"Tue".into(), &"EEE".into())).to(be_ok());
    expect!(validate_datetime(&"Tuesday".into(), &"EEE".into())).to(be_ok());
    expect!(validate_datetime(&"3".into(), &"u".into())).to(be_ok());
    expect!(validate_datetime(&"32".into(), &"u".into())).to(be_err());
    expect!(validate_datetime(&"0".into(), &"u".into())).to(be_err());
  }

}