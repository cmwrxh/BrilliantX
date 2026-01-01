use std::fs;
use std::process::Command;

fn main() {
    println!("🚀 cmwrxh: Building January 2026 Contribution Wall...");

    let year = 2026;
    let month = 1;
    let folder = format!("project/{}/january", year);
    fs::create_dir_all(&folder).ok();

    // Loop through all 31 days of January
    for day in 1..=31 {
        // Calculate weekday (0=Sun, 1=Mon, ..., 6=Sat) 
        // Sakamoto's algorithm for quick date math
        let weekday = calc_weekday(year, month, day);

        let commit_count = match weekday {
            0 => 0,  // Sunday: 0 commits
            1 => 10, // Monday: 10 commits
            2 => 9,  // Tuesday: 9 commits
            _ => 8,  // Wed-Sat: At least 8 commits
        };

        if commit_count > 0 {
            for i in 1..=commit_count {
                let date = format!("{}-{:02}-{:02} 10:{:02}:00 +0000", year, month, day, i + 10);
                let file_name = format!("{}/day_{}_task_{}.rs", folder, day, i);
                fs::write(&file_name, "// January Sprint").ok();
                run_git_commit(&date, &format!("feat: jan day {} update {}", day, i));
            }
            println!("📅 Jan {}: Generated {} commits", day, commit_count);
        }
    }

    println!("\n✅ Done! Use: git push origin main --force");
}

fn calc_weekday(y: i32, m: i32, d: i32) -> i32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = y;
    if m < 3 { y -= 1; }
    (y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + d) % 7
}

fn run_git_commit(date: &str, msg: &str) {
    std::env::set_var("GIT_AUTHOR_DATE", date);
    std::env::set_var("GIT_COMMITTER_DATE", date);
    let _ = Command::new("git").args(["add", "."]).output();
    let _ = Command::new("git").args(["commit", "-m", msg]).output();
}