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
use std::fs::{self, metadata, read_dir, ReadDir};

pub struct File<'a> {
    name: &'a str,
    parent: &'a Directory<'a>
}

pub struct Directory<'a> {
    name: &'a str,
    files: &'a [File<'a>],
    parent: &'a Directory<'a>
}

pub fn visit_dirs(dir: &Path) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        print_contents(try!(fs::read_dir(dir)));
    }
    Ok(())
}

fn print_contents(dir: ReadDir) -> io::Result<()> {
    for file in dir {
        let entry = try!(file);
        println!("{:?}", entry.path());
    }
    Ok(())
}
