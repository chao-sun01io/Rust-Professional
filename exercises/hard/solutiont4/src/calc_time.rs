// It only works for 2025

use std::{collections::HashSet, fmt::format};

pub fn time_info(input: &str) -> String {
    // input example : 2025-01-18

    let (year, month, day) = parse_date(input);
    let nth_day = nth_day_of_year(year, month, day);
    let week_number = nth_week_of_2025(nth_day);
    let weekday = weekday_of_2025(nth_day);
    let days_left = days_left_of_year(year, month, day);
    let days_to_festival = days_left_to_next_spring_festival_2025(year, month, day);
    let days_to_market = days_to_next_market_open_day(year, month, day);

    // Using format! macro
    format!(
        "{},{},{},{},{},{}",
        week_number, weekday,nth_day,days_left, days_to_festival, days_to_market
    )
}

fn parse_date(input: &str) -> (u32, u32, u32) {
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 3 {
        panic!("Invalid input")
    }

    let year = parts[0].parse::<u32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();
    // Validate month range
    // if month < 1 || month > 12 {
    //     return None;
    // }
    (year, month, day)
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: u32, month: u32) -> u32 {
    let days_in_month = match month {
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 31,
    };

    days_in_month
}

fn nth_day_of_year(year: u32, month: u32, day: u32) -> u32 {
    let mut total_days = 0;
    for m in 1..month {
        total_days += days_in_month(year, m);
    }

    total_days += day;
    total_days
}

fn days_left_of_year(year: u32, month: u32, day: u32) -> u32 {
    if is_leap_year(year) {
        366 - nth_day_of_year(year, month, day)
    } else {
        365 - nth_day_of_year(year, month, day)
    }
}

fn nth_week_of_2025(nth_day_of_year: u32) -> u32 {
    //  2025-01-01, Wed, the 1st week
    //ISO8061: Monday the first day of the week

    if nth_day_of_year <= 5 {
        1
    } else if nth_day_of_year >=363 {
        1 // 1st week of 2026
    } else{
        ((nth_day_of_year - 5) as f32 / 7.0).ceil() as u32 + 1
    }
}

fn weekday_of_2025(nth_day_of_year: u32) -> u32 {
   let n = (nth_day_of_year % 7 + 2) % 7;
   if n == 0 {
       7
   }else {
       n
   }
}

fn days_left_to_next_spring_festival_2025(year: u32, month: u32, day: u32) -> u32 {
    let chinese_new_year_2025 = nth_day_of_year(2025, 1, 29);
    let days = nth_day_of_year(year, month, day);
    if days <= chinese_new_year_2025 {
        // 2025 Spring Festival
        chinese_new_year_2025 - days
    } else {
        let chinese_new_year_2026 = nth_day_of_year(2026, 2, 17);
        chinese_new_year_2026 + days_left_of_year(year, month, day)
    }
}

fn days_to_next_market_open_day(year: u32, month: u32, day: u32) -> u32 {
    const CLOSED_DAY: [&str; 19] = [
        "2025-01-01",
        "2025-01-28",
        "2025-01-29",
        "2025-01-30",
        "2025-01-31",
        "2025-02-03",
        "2025-02-04",
        "2025-04-04",
        "2025-05-01",
        "2025-05-02",
        "2025-05-05",
        "2025-06-02",
        "2025-10-01",
        "2025-10-02",
        "2025-10-03",
        "2025-10-06",
        "2025-10-07",
        "2025-10-08",
        "2026-01-01",
    ];

    // Create a HashSet to store closed days
    let mut closed_days: HashSet<u32> = HashSet::new();

    // Parse and insert all closed days into the HashSet
    for &date_str in CLOSED_DAY.iter() {
        let datetuple = parse_date(date_str);
        closed_days.insert(nth_day_of_year(datetuple.0, datetuple.1, datetuple.2));
    }

    let d: u32 = nth_day_of_year(year, month, day);
 
    let mut next_open_day = 367;
    // 2025
    for next in d+1..365 {
        let weekday = weekday_of_2025(next);
        if weekday == 6 || weekday == 7 || closed_days.contains(&next) {
            continue;
        } else {
            next_open_day = next;
            break;
        }
    }

    if d == 365 {
        next_open_day = 367
    }
    // println!("{} - {}", d, next_open_day);
    next_open_day - d - 1
}
