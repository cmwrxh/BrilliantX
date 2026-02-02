use std::fs;
use std::process::Command;

fn main() {
    println!("🚀 cmwrxh: Filling February 2026...");
    let year = 2026;
    let month = 2; // February
    let folder = "project/2026/february";
    fs::create_dir_all(folder).ok();

    for day in 1..=28 {
        // Monday=1, Tuesday=2, etc. (Feb 1st 2026 is a Sunday=0)
        let weekday = calc_weekday(year, month, day);
        
        let commit_count = match weekday {
            0 => 0,  // Sunday: Rest day
            1 => 10, // Monday: High intensity
            2 => 9,  // Tuesday: High intensity
            _ => 8,  // Others: Steady work
        };

        for i in 1..=commit_count {
            let date = format!("{}-02-{:02} 12:{:02}:00 +0000", year, day, i + 10);
            let file_path = format!("{}/day_{}_commit_{}.rs", folder, day, i);
            fs::write(&file_path, format!("// cmwrxh Feb {} - Part {}", day, i)).ok();
            
            run_git_commit(&date, &format!("feat: feb development day {} iteration {}", day, i));
        }
    }
    println!("✅ February is ready to be pushed!");
}

fn calc_weekday(y: i32, m: i32, d: i32) -> i32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = y; if m < 3 { y -= 1; }
    (y + y/4 - y/100 + y/400 + t[(m-1) as usize] + d) % 7
}

fn run_git_commit(date: &str, msg: &str) {
    std::env::set_var("GIT_AUTHOR_DATE", date);
    std::env::set_var("GIT_COMMITTER_DATE", date);
    Command::new("git").args(["add", "."]).output().ok();
    Command::new("git").args(["commit", "-m", msg]).output().ok();
}