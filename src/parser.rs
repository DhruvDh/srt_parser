use anyhow::{
    Context,
    Result,
};
use time::Time;

peg::parser! {
  /// A grammar for the `srt` subtitle format
  pub grammar srt() for str {
    /// matches any sequeuce of 1 or more numbers
    rule number() -> u32
        = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    /// matches any number of whitespace characters
    rule whitespace() = quiet!{[' ' | '\n' | '\t' | '\r']+}

    /// matches a sequence number for a subtitle
    pub rule sequence_number() -> u32
        = n:number() whitespace()? { n }

    /// matches a timecode in the format HH:MM:SS,mmm
    rule time() -> Result<Time>
        = h:number() ":" m:number() ":" s:number() "," ms:number() {
            let h: u8 = h.try_into().context("Could not parse hour as u8")?;
            let m: u8 = m.try_into().context("Could not parse minutes as u8")?;
            let s: u8 = s.try_into().context("Could not parse seconds as u8")?;
            let ms: u16 = ms.try_into().context("Could not parse milliseconds as u16")?;

            Time::from_hms_milli(h, m, s, ms).context("Could not parse time")
         }

    /// matches a timecode range in the format HH:MM:SS,mmm --> HH:MM:SS,mmm
    pub rule sub_duration() -> Result<(Time, Time)>
        = start:time() whitespace()? "-->" whitespace()? end:time() {
            Ok((start.context("When parsing start time")?, end.context("When parsing end time")?))
        }
  }
}
