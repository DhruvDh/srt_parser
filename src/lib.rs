#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! A simple SubRip file parser. Example usage -
//!
//! ```rust
//! use std::path::PathBuf;
//!
//! use srt_parser::SubRipFile;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let subs = SubRipFile::new(PathBuf::from("test.srt"))?;
//!
//!     for sub in subs.subtitles() {
//!         println!("{:#?}", sub);
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::path::PathBuf;

use anyhow::{
    anyhow,
    Context,
    Result,
};
use itertools::Itertools;
use time::Time;

/// Includes `peg` parsers used to parse subtitles
pub mod parser;

/// A struct representing a subtitle file
#[derive(Debug)]
pub struct SubRipFile {
    /// The path to the subtitle file
    path:      PathBuf,
    /// The source text of the subtitle file
    source:    String,
    /// a vector of subtitles parsed from the file
    subtitles: Vec<Subtitle>,
}

impl SubRipFile {
    /// Creates a new `SubTitleFile` from a path to a subtitle file
    pub fn new(path: PathBuf) -> Result<Self> {
        let source = std::fs::read_to_string(&path)
            .context(format!("Failed to read file as string {}", &path.display()))?;
        let subtitles = source
            .lines()
            .into_iter()
            .map(String::from)
            .map(|line| line.replace('\u{feff}', ""))
            .coalesce(|prev, next| {
                if next.trim().is_empty() {
                    Err((prev, next))
                } else {
                    Ok(format!("{}\n{}", prev, next).trim().to_string())
                }
            })
            .filter(|s| !s.trim().is_empty())
            .map(Subtitle::from_string)
            .collect::<Result<Vec<Subtitle>>>()?;

        Ok(Self {
            path,
            source,
            subtitles,
        })
    }

    /// Returns the path to the subtitle file
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns the source text of the subtitle file
    pub fn source(&self) -> &str {
        self.source.as_ref()
    }

    /// Returns a vector of subtitles parsed from the file
    pub fn subtitles(&self) -> &[Subtitle] {
        self.subtitles.as_ref()
    }
}

/// A struct representing a subtitle
#[derive(Debug, Clone)]
pub struct Subtitle {
    /// The sequence number of the subtitle
    sequence_number: u32,
    /// The start timecode of the subtitle
    start:           Time,
    /// The end timecode of the subtitle
    end:             Time,
    /// The text of the subtitle
    text:            String,
}

impl Subtitle {
    /// Creates a new `SubTitle`
    pub fn new(sn: u32, s: Time, e: Time, t: String) -> Self {
        Self {
            sequence_number: sn,
            start:           s,
            end:             e,
            text:            t,
        }
    }

    /// Creates a new `SubTitle` from a string
    pub fn from_string(source: String) -> Result<Self> {
        let lines = source.lines().collect::<Vec<&str>>();
        if lines.len() < 3 {
            return Err(anyhow!(
                "Invalid subtitle (length is {}): {:?}",
                lines.len(),
                lines
            ));
        }
        let sequence_number = parser::srt::sequence_number(lines[0].trim()).context(format!(
            "Could not parse a sequence number in the SRT file: {:?}",
            lines[0]
        ))?;
        let (start, end) = parser::srt::sub_duration(lines[1].trim())
            .context("Could not parse a timecode in the SRT file")??;
        let text = lines[2..].join("\n");

        Ok(Self {
            sequence_number,
            start,
            end,
            text,
        })
    }

    /// Returns the sequence number of the subtitle
    pub fn sequence_number(&self) -> u32 {
        self.sequence_number
    }

    /// Returns the start timecode of the subtitle
    pub fn start(&self) -> Time {
        self.start
    }

    /// Returns the end timecode of the subtitle
    pub fn end(&self) -> Time {
        self.end
    }

    /// Returns the text of the subtitle
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }
}
