use chrono::{DateTime, Local};
use std::{
    fs::{DirEntry, Metadata},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

pub fn get_file_type(mode: u32) -> char {
    match mode & libc::S_IFMT {
        libc::S_IFDIR => 'd',
        libc::S_IFLNK => 'l',
        _ => '-',
    }
}

pub fn get_mode(metadata: &Metadata) -> String {
    let mode = metadata.mode();
    let file_type = get_file_type(mode);

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

pub fn get_user(metadata: &Metadata) -> Option<String> {
    let uid = metadata.uid();
    let user = users::get_user_by_uid(uid)?;
    let user_name = user.name().to_str()?.to_string();

    Some(user_name)
}

pub fn get_group(metadata: &Metadata) -> Option<String> {
    let gid = metadata.gid();
    let group = users::get_group_by_gid(gid)?;
    let group_name = group.name().to_str()?.to_string();

    Some(group_name)
}

pub fn get_size(metadata: &Metadata) -> String {
    metadata.len().to_string()
}

pub fn get_time(metadata: &Metadata) -> Option<String> {
    let time: DateTime<Local> = metadata.modified().ok()?.into();
    let time_formatted = time.format("%b %e %H:%M").to_string();

    Some(time_formatted)
}

pub fn get_nlink(metadata: &Metadata) -> String {
    metadata.nlink().to_string()
}

pub fn get_xattr(path: &PathBuf) -> Option<String> {
    let xattr = xattr::list(path).ok()?.next()?;
    let xattr_suffix = match xattr.to_str()? {
        "system.posix_acl_access" | "system.posix_acl_default" => "+",
        "security.selinux" => ".",
        _ => "",
    }
    .to_string();

    Some(xattr_suffix)
}

pub fn get_is_hidden(file_name: &str) -> bool {
    file_name.starts_with('.')
}

pub fn get_metadata(file: &DirEntry) -> Option<Metadata> {
    file.metadata().ok()
}

pub fn get_path_string(path: &PathBuf) -> Option<String> {
    let path_string = path.to_str()?.to_string();

    Some(path_string)
}

pub fn get_path(file: &DirEntry) -> PathBuf {
    file.path()
}

pub fn get_file_name(file: &DirEntry) -> Option<String> {
    file.file_name().into_string().ok()
}
