use std::fs::metadata;
use std::path::Path;
use filetime::FileTime;
use chrono::NaiveDateTime;


/// returns the last time the file was modified as String type
pub fn get_mtime(file: &Path) -> String {
    let metadata = metadata(file).unwrap();
    let mtime = FileTime::from_last_modification_time(&metadata).seconds();

    // formatted mtime
    NaiveDateTime::from_timestamp_opt(mtime, 0).unwrap()
                                            .format("%Y-%m-%d %H:%M:%S")
                                            .to_string()

}
