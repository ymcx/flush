use crate::flags::Flags;
use chrono::{DateTime, Local};
use std::{
    fs::{DirEntry, Metadata},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

mod flags;

fn get_mode(metadata: &Metadata) -> String {
    let mode = metadata.mode();
    let file_type = match mode & libc::S_IFMT {
        libc::S_IFDIR => 'd',
        libc::S_IFLNK => 'l',
        _ => '-',
    };

    let permissions = [
        (libc::S_IRUSR, 'r'),
        (libc::S_IWUSR, 'w'),
        (libc::S_IXUSR, 'x'),
        (libc::S_IRGRP, 'r'),
        (libc::S_IWGRP, 'w'),
        (libc::S_IXGRP, 'x'),
        (libc::S_IROTH, 'r'),
        (libc::S_IWOTH, 'w'),
        (libc::S_IXOTH, 'x'),
    ];

    let mut mode_string = String::from(file_type);
    for (bit, char) in permissions {
        let permission = if mode & bit != 0 { char } else { '-' };
        mode_string.push(permission);
    }

    mode_string
}

fn get_user(metadata: &Metadata) -> Option<String> {
    let uid = metadata.uid();
    let user = users::get_user_by_uid(uid)?;
    let user_name = user.name().to_str()?.to_string();

    Some(user_name)
}

fn get_group(metadata: &Metadata) -> Option<String> {
    let gid = metadata.gid();
    let group = users::get_group_by_gid(gid)?;
    let group_name = group.name().to_str()?.to_string();

    Some(group_name)
}

fn get_size(metadata: &Metadata) -> String {
    metadata.len().to_string()
}

fn get_time(metadata: &Metadata) -> Option<String> {
    let time: DateTime<Local> = metadata.modified().ok()?.into();
    let time_formatted = time.format("%b %e %H:%M").to_string();

    Some(time_formatted)
}

fn get_nlink(metadata: &Metadata) -> String {
    metadata.nlink().to_string()
}

fn get_xattr(path: &PathBuf) -> Option<String> {
    let xattr = xattr::list(path).ok()?.next()?;
    let xattr_suffix = match xattr.to_str()? {
        "system.posix_acl_access" | "system.posix_acl_default" => "+",
        "security.selinux" => ".",
        _ => "",
    }
    .to_string();

    Some(xattr_suffix)
}

fn get_is_hidden(file_name: &str) -> bool {
    file_name.starts_with('.')
}

fn get_metadata(file: &DirEntry) -> Option<Metadata> {
    file.metadata().ok()
}

fn get_path_string(path: &PathBuf) -> Option<String> {
    let path_string = path.to_str()?.to_string();

    Some(path_string)
}

fn get_path(file: &DirEntry) -> PathBuf {
    file.path()
}

fn get_file_name(file: &DirEntry) -> Option<String> {
    file.file_name().into_string().ok()
}

fn get_file_string(file: &DirEntry, flags: &Flags) -> Option<Vec<String>> {
    let file_name = get_file_name(file)?;
    let metadata = get_metadata(file)?;
    let path = get_path(file);

    let is_hidden = get_is_hidden(&file_name);

    if !flags.all && is_hidden {
        return None;
    }

    let path_string = get_path_string(&path)?;
    let xattr = get_xattr(&path)?;

    let group = get_group(&metadata)?;
    let nlink = get_nlink(&metadata);
    let mode = get_mode(&metadata);
    let size = get_size(&metadata);
    let time = get_time(&metadata)?;
    let user = get_user(&metadata)?;

    let file_string = match (flags.long, flags.full_path) {
        (false, false) => vec![file_name],
        (false, true) => vec![path_string],
        (true, false) => vec![mode, xattr, nlink, user, group, size, time, file_name],
        (true, true) => vec![mode, xattr, nlink, user, group, size, time, path_string],
    };

    Some(file_string)
}

fn find_column_widths(rows: &Vec<Vec<String>>) -> Option<Vec<usize>> {
    let column_amount = rows.first()?.len();
    let mut column_widths = vec![0; column_amount];

    for row in rows {
        for (column, item) in row.iter().enumerate() {
            column_widths[column] = column_widths[column].max(item.len());
        }
    }

    Some(column_widths)
}

fn format_table(rows: &Vec<Vec<String>>) -> Option<String> {
    let column_widths = find_column_widths(rows)?;
    let mut table = String::new();

    for row in rows {
        for (column, item) in row.iter().enumerate() {
            let starts_with_digit = item.chars().next()?.is_ascii_digit();
            let padded = if starts_with_digit {
                format!("{:>width$}", item, width = column_widths[column])
            } else {
                format!("{:width$}", item, width = column_widths[column])
            };

            table.push_str(&padded);
            if column != 0 && column != row.len() - 1 {
                table.push(' ');
            }
        }

        table.push('\n');
    }

    table.pop();
    Some(table)
}

fn get_files_from_directory(directory: &str, flags: &Flags) -> Option<String> {
    if directory.is_empty() {
        let directory = common::get_current_directory();
        return get_files_from_directory(&directory, flags);
    }

    let path = Path::new(directory);
    if !path.exists() {
        return None;
    }

    let mut all_files = Vec::new();
    if let Ok(files) = Path::read_dir(path) {
        for file in files {
            let file_string = get_file_string(&file.ok()?, flags);

            if let Some(file_string) = file_string {
                all_files.push(file_string);
            }
        }
    }

    let table = format_table(&all_files).unwrap_or_default();
    Some(table)
}

fn get_files_from_directories(directories: &Vec<String>, flags: &Flags) -> Option<String> {
    if directories.len() == 0 {
        return get_files_from_directory("", flags);
    }

    let all_files: Vec<Option<String>> = directories
        .iter()
        .map(|directory| get_files_from_directory(directory, flags))
        .collect();

    let found_all = all_files.iter().all(|result| result.is_some());
    if !found_all {
        return None;
    }

    let files: Vec<String> = all_files.into_iter().filter_map(|result| result).collect();
    let files = files.join("\n");

    Some(files)
}

fn main() {
    let (arguments, flags) = common::read_arguments();
    let flags = Flags::new(&flags);
    let files = get_files_from_directories(&arguments, &flags);

    if let Some(files) = files {
        println!("{}", files);
    } else {
        println!("No such file or directory");
    }
}
