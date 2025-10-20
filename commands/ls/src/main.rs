use crate::flags::Flags;
use chrono::{DateTime, Local};
use std::{
    fs::{DirEntry, Metadata},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
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

fn get_file_string(
    permissions: &str,
    xattr: &str,
    nlink: &str,
    user: &str,
    group: &str,
    size: &str,
    time: &str,
    file: &str,
) -> String {
    format!(
        "{}{} {} {} {} {} {} {}",
        permissions, xattr, nlink, user, group, size, time, file,
    )
}

fn get_file_string_complete(file: &DirEntry, flags: &Flags) -> Option<String> {
    let file_name = get_file_name(file)?;
    let metadata = get_metadata(file)?;
    let path = get_path(file);

    let xattr = get_xattr(&path)?;
    let path_string = get_path_string(&path)?;

    let is_hidden = get_is_hidden(&file_name);

    if !flags.all && is_hidden {
        return None;
    }

    let group = get_group(&metadata)?;
    let nlink = get_nlink(&metadata);
    let permissions = get_permissions(&metadata);
    let size = get_size(&metadata);
    let time = get_time(&metadata)?;
    let user = get_user(&metadata)?;

    let file_name_long = get_file_string(
        &permissions,
        &xattr,
        &nlink,
        &user,
        &group,
        &size,
        &time,
        &file_name,
    );
    let path_string_long = get_file_string(
        &permissions,
        &xattr,
        &nlink,
        &user,
        &group,
        &size,
        &time,
        &path_string,
    );

    match (flags.long, flags.full_path) {
        (false, false) => Some(file_name),
        (true, false) => Some(file_name_long),
        (false, true) => Some(path_string),
        (true, true) => Some(path_string_long),
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
            let file_string = get_file_string_complete(&file, flags);

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
