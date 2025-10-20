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
    let owner = metadata.uid();
    let owner = users::get_user_by_uid(owner).unwrap();
    let owner = owner.name().to_str().unwrap().to_string();
    owner
}

fn get_group(metadata: &Metadata) -> String {
    let owner2 = metadata.gid();
    let owner2 = users::get_group_by_gid(owner2).unwrap();
    let owner2 = owner2.name().to_str().unwrap().to_string();
    owner2
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
    let owner = get_user(&metadata);
    let owner2 = get_group(&metadata);
    let size = get_size(&metadata);
    let time = get_time(&metadata);
    let nd = get_nlink(&metadata);

    let file_name_long = format!(
        "{}{} {} {} {} {} {} {}",
        permissions, xatt, nd, owner, owner2, size, time, file_name,
    );

    let path_long = format!(
        "{}{} {} {} {} {} {} {}",
        permissions, xatt, nd, owner, owner, size, time, path,
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
        Some("".to_string())
    }
}

pub fn get_files_from_directory(directory: &str, flags: &Flags) -> Option<Vec<String>> {
    let directory = if directory.is_empty() {
        common::get_current_directory()
    } else {
        directory.to_string()
    };

    let full_path = Path::new(&directory);
    if !full_path.exists() {
        return None;
    }

    let mut all_files = Vec::new();
    if let Ok(files) = Path::read_dir(full_path) {
        for file in files {
            let file = file.unwrap();
            if let Some(file_string) = get_file_string(&file, flags) {
                all_files.push(file_string);
            }
        }
    }

    Some(all_files)
}

pub fn get_files_from_directories(directories: &Vec<String>, flags: &Flags) -> Option<Vec<String>> {
    if directories.len() == 0 {
        return get_files_from_directory("", flags);
    }

    let all_files: Vec<Option<Vec<String>>> = directories
        .iter()
        .map(|directory| get_files_from_directory(directory, flags))
        .collect();

    let found_all = all_files.iter().all(|result| result.is_some());

    let collected: Vec<String> = all_files
        .into_iter()
        .filter_map(|result| result)
        .flatten()
        .collect();

    if found_all {
        Some(collected)
    } else {
        None
    }
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
