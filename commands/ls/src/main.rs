use crate::flags::Flags;
use chrono::{DateTime, Local};
use std::{
    fs::{DirEntry, Metadata},
    os::unix::fs::MetadataExt,
    path::Path,
};

mod flags;

fn get_permissions(metadata: &Metadata) -> String {
    let mode = metadata.mode();
    let file_type = match mode & libc::S_IFMT {
        libc::S_IFDIR => 'd',
        libc::S_IFLNK => 'l',
        _ => '-',
    };

    let perms = [
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

    let mut result = String::with_capacity(10);
    result.push(file_type);
    for (bit, ch) in perms {
        result.push(if mode & bit != 0 { ch } else { '-' });
    }

    result
}

fn get_user(metadata: &Metadata) -> String {
    let user = metadata.uid();
    let user = users::get_user_by_uid(user).unwrap();
    let user = user.name().to_str().unwrap().to_string();
    user
}

fn get_group(metadata: &Metadata) -> String {
    let group = metadata.gid();
    let group = users::get_group_by_gid(group).unwrap();
    let group = group.name().to_str().unwrap().to_string();
    group
}

fn get_size(metadata: &Metadata) -> String {
    metadata.len().to_string()
}

fn get_time(metadata: &Metadata) -> String {
    let time = metadata.modified().unwrap();
    let time: DateTime<Local> = time.into();
    let time = time.format("%b %e %H:%M").to_string();
    time
}

fn get_nlink(metadata: &Metadata) -> String {
    metadata.nlink().to_string()
}

fn get_xattr(file: &DirEntry) -> String {
    let xatt = xattr::list(&file.path()).unwrap().next().unwrap();
    let xatt = xatt.to_string_lossy();
    let xatt = match xatt.to_string().as_str() {
        "security.selinux" => ".",
        "system.posix_acl_access" | "system.posix_acl_default" => "+",
        _ => "",
    };
    xatt.to_string()
}

fn get_is_hidden(file: &DirEntry) -> bool {
    file.file_name().into_string().unwrap().starts_with('.')
}

fn get_metadata(file: &DirEntry) -> Metadata {
    file.metadata().unwrap()
}

fn get_path(file: &DirEntry) -> String {
    file.path()
        .into_os_string()
        .into_string()
        .unwrap_or_default()
}

fn get_file_name(file: &DirEntry) -> String {
    file.file_name()
        .into_string()
        .unwrap_or_default()
        .to_string()
}

fn get_file_string(file: &DirEntry, flags: &Flags) -> Option<String> {
    let path = get_path(file);
    let file_name = get_file_name(file);
    let is_hidden = get_is_hidden(file);
    let xatt = get_xattr(file);
    let metadata = get_metadata(file);

    let permissions = get_permissions(&metadata);
    let user = get_user(&metadata);
    let group = get_group(&metadata);
    let size = get_size(&metadata);
    let time = get_time(&metadata);
    let nd = get_nlink(&metadata);

    let file_name_long = format!(
        "{}{} {} {} {} {} {} {}",
        permissions, xatt, nd, user, group, size, time, file_name,
    );

    let path_long = format!(
        "{}{} {} {} {} {} {} {}",
        permissions, xatt, nd, user, group, size, time, path,
    );

    if !flags.all && is_hidden {
        None
    } else if !flags.long && !flags.full_path {
        Some(file_name)
    } else if !flags.long && flags.full_path {
        Some(path)
    } else if flags.long && !flags.full_path {
        Some(file_name_long)
    } else if flags.long && flags.full_path {
        Some(path_long)
    } else {
        None
    }
}

fn get_files_from_directory(directory: &str, flags: &Flags) -> Option<Vec<String>> {
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
            let file = file.unwrap();
            let file_string = get_file_string(&file, flags);

            if let Some(file_string) = file_string {
                all_files.push(file_string);
            }
        }
    }

    Some(all_files)
}

fn get_files_from_directories(directories: &Vec<String>, flags: &Flags) -> Option<Vec<String>> {
    if directories.len() == 0 {
        return get_files_from_directory("", flags);
    }

    let all_files: Vec<Option<Vec<String>>> = directories
        .iter()
        .map(|directory| get_files_from_directory(directory, flags))
        .collect();

    let found_all = all_files.iter().all(|result| result.is_some());
    if !found_all {
        return None;
    }

    let files = all_files
        .into_iter()
        .filter_map(|result| result)
        .flatten()
        .collect();

    Some(files)
}

fn main() {
    let (arguments, flags) = common::read_arguments();
    let flags = Flags::new(&flags);
    let files = get_files_from_directories(&arguments, &flags);

    if let Some(files) = files {
        println!("{}", files.join("\n"));
    } else {
        println!("No such file or directory");
    }
}
