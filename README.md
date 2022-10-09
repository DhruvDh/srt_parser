# `srt_parser`

 A simple SubRip file parser. Example usage -
 
 ```rust
 use std::path::PathBuf;
 use srt_parser::SubRipFile;
 
 fn main() -> Result<(), Box<dyn std::error::Error>> {
     let subs = SubRipFile::new(PathBuf::from("test.srt"))?;
     
     for sub in subs.subtitles() {
       println!("{:#?}", sub);
     }
 
     Ok(())
 }
```