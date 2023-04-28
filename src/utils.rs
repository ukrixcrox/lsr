use std::fs::metadata;
use std::path::PathBuf;
use filetime::FileTime;
use chrono::NaiveDateTime;
use std::os::unix::fs::PermissionsExt;
use unix_mode;

/// returns the last time the file was modified as String type
pub fn get_mtime(file: &PathBuf) -> String {
    let metadata = metadata(file).unwrap();
    let mtime = FileTime::from_last_modification_time(&metadata).seconds();

    // formatted mtime
    NaiveDateTime::from_timestamp_opt(mtime, 0).unwrap()
                                            .format("%Y-%m-%d %H:%M:%S")
                                            .to_string()

}

/// returns the file size as String type
pub fn get_size(file: &PathBuf) -> String{
    metadata(file).unwrap().len().to_string()
}

/// returns the biggest len value of a Vec<usize> as usize type
/// with time complexity of O(n)
pub fn get_longest_len(len: Vec<usize>) -> usize{
    let mut longest_len :usize = 0;

    for element in len{
        if element > longest_len{
            longest_len = element;
        }
    }
    return longest_len
}

/// get the file permissons as String type
pub fn get_file_permissions(file: &PathBuf) -> String{
    let permissions = file.metadata().unwrap().permissions();

    unix_mode::to_string(permissions.mode())
}

