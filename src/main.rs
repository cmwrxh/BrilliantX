use std::fs;
use std::process::Command;

fn main() {
    println!("🚀 cmwrxh: Initiating High-Volume Sprint (Target: Top 10)...");
    
    // We will run this for BOTH January and February to maximize your count
    for month in 1..=2 {
        let year = 2026;
        let days_in_month = if month == 1 { 31 } else { 28 };
        let folder = format!("project/2026/m{}", month);
        fs::create_dir_all(&folder).ok();

        for day in 1..=days_in_month {
            let weekday = calc_weekday(year, month, day);
            
            // High volume: 25-30 commits per day
            let commit_count = match weekday {
                0 => 0,  // Sunday rest (keeps it looking human)
                1 => 35, // Power Mondays
                2 => 30, // Strong Tuesdays
                _ => 25, // Steady weekday/Saturday
            };

            for i in 1..=commit_count {
                let date = format!("{}-{:02}-{:02} 14:{:02}:00 +0000", year, month, day, i % 60);
                let file_path = format!("{}/d{}_c{}.rs", folder, day, i);
                fs::write(&file_path, format!("// cmwrxh Top 10 Rank Push - Month {} Day {}", month, day)).ok();
                
                run_git_commit(&date, &format!("feat: system optimization month {} day {} iteration {}", month, day, i));
            }
        }
    }
    println!("✅ 2026 Data Expanded! Ready to Push.");
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