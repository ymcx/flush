use crate::flags::Flags;
use common::file;
use std::{fs::DirEntry, path::Path};

mod flags;

fn get_file_string(file: &DirEntry, flags: &Flags) -> Option<Vec<String>> {
    let file_name = file::get_file_name(file)?;
    let metadata = file::get_metadata(file)?;
    let path = file::get_path(file);

    let is_hidden = file::get_is_hidden(&file_name);

    if !flags.all && is_hidden {
        return None;
    }

    let path_string = file::get_path_string(&path)?;
    let xattr = file::get_xattr(&path)?;

    let group = file::get_group(&metadata)?;
    let nlink = file::get_nlink(&metadata);
    let mode = file::get_mode(&metadata);
    let size = file::get_size(&metadata);
    let time = file::get_time(&metadata)?;
    let user = file::get_user(&metadata)?;

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
        let directory = common::get_current_directory()?;
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

    if flags.help {
        println!(
            "{}\n\n{}\n{}\n{}\n{}",
            "Usage: ls [OPTIONS] [FILES]",
            "-a  show files/directories starting with a dot",
            "-f  show the full path instead of the filename",
            "-l  show additional information",
            "-h  display this epic document",
        );

        return;
    }

    let files = get_files_from_directories(&arguments, &flags);
    if let Some(files) = files {
        println!("{}", files);
    } else {
        println!("No such file or directory");
    }
}
