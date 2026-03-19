use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let now = Instant::now();
}
fn count_lines_sync(path: &Path) -> i32 {
    let mut count = 0;
    if let Ok(lines) = read_lines(path) {
        lines.for_each(|line| {
            if let Ok(line) = line
                && !line.trim().is_empty()
            {
                count += 1;
            }
        })
    }
    count
}
fn read_lines<T>(file_name: T) -> anyhow::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
