// sink file system module
// Copyright (C) 2015 Chuy Del Castillo <chuy@imchuy.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use std::path::Path;
use std::io;
use std::fs::{self, metadata, read_dir, ReadDir, DirEntry};
use std::ptr::null;

pub trait Entry<'a> {
    fn get_name(self) -> &'a str;
    fn is_dir(self) -> bool;
}

pub struct File<'a> {
    name: &'a str,
    path: &'a Path,
    parent: Option<Directory<'a>>
}

pub struct Directory<'a> {
    name: &'a str,
    path: &'a Path,
    parent: Option<Box<Directory<'a>>>,
    children: Option<Vec<Box<Entry<'a>>>>
}

impl<'a> Entry<'a> for File<'a> {
    fn get_name(self) -> &'a str {
        self.name;
    }

    fn is_dir(self) -> bool {
        false;
    }
}

impl<'a> Entry<'a> for Directory<'a> {
    fn get_name(self) -> &'a str {
        self.name;
    }

    fn is_dir(self) -> bool {
        true;
    }
}

pub fn set_root_dir(dir_path: &str) -> io::Result<Directory> {
    let dir = Path::new(dir_path);
    if try!(fs::metadata(dir)).is_dir() {
        let root = Directory { name: dir_path, path: dir, parent: None, children: None };
        return Ok(root);
    }
    panic!("Not a directory");
}

// This method screams for abstraction, but not sure how to do it right now..
fn inspect_dir<'a>(dir: &'a mut Directory) -> io::Result<Directory<'a>> {
    for file in try!(fs::read_dir(dir.path)) {
        let entry = try!(file);
        if try!(entry.metadata()).is_dir() {
            let child = Directory {
                name: entry.file_name().to_str().unwrap(),
                path: entry.path().as_path(),
                parent: Some(Box::from_raw(dir)),
                children: None
            };
            inspect_dir(&mut child);

            if(dir.children.is_some()) {
                dir.children.unwrap().push(Box::from_raw(&mut child));
            } else {
                dir.children = Some(vec![Box::from_raw(&mut child)]);
            }
        } else {
            let child = File {
                name: entry.file_name().to_str().unwrap(),
                path: entry.path().as_path(),
                parent: Some(*dir)
            };

            if(dir.children.is_some()) {
                dir.children.unwrap().push(Box::from_raw(&mut child));
            } else {
                dir.children = Some(vec![Box::from_raw(&mut child)]);
            }
        }

    }
    Ok(dir);
}
