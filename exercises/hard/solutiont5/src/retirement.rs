use core::fmt;

use chrono::{Datelike, NaiveDate, Months};

struct Retirement{
    retirement_date: NaiveDate,
    retirement_age: f32,
    extended_work_month: u32
}

impl fmt::Display for Retirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
               // Check if the number is effectively a whole number
        let age_str = if (self.retirement_age * 100.0).round() % 100.0 == 0.0 {
            format!("{}", self.retirement_age as u32)
        } else {
            format!("{:.2}", self.retirement_age)
        };

        write!(f,"{},{},{}",self.retirement_date.format("%Y-%m"),age_str,self.extended_work_month)
    }
}

pub fn retire_time(dob: &str, retire_type: &str) -> String {
   let full_date_str = format!("{}-01", dob);
   let date_of_birth = NaiveDate::parse_from_str(&full_date_str, "%Y-%m-%d").unwrap();
   match retire_type {
       "男职工" => calculate_retirement(date_of_birth, 60,4,1, 36).to_string(),
       "原法定退休年龄55周岁女职工" => calculate_retirement(date_of_birth,55,4,1,36).to_string(),
       "原法定退休年龄50周岁女职工" => calculate_retirement(date_of_birth, 50,2,1,60).to_string(),
       _  => panic!("Invalid retirement type")
   }
}

fn calculate_retirement(dob: NaiveDate, origin_retire_age: u32, every_n_month: u32, by: u32,max_extension: u32)-> Retirement{
    let origin_retirement = origin_retire_date(dob,origin_retire_age);
    let extended_work_month = extend_every_n_month_by(&origin_retirement, NaiveDate::from_ymd_opt(2025, 01, 01).unwrap(), every_n_month, by,max_extension );
    let retirement_date = origin_retirement.checked_add_months(Months::new(extended_work_month)).unwrap();
    // Calculate actual retirement age in years and months
    let total_months = (retirement_date.year() - dob.year()) * 12 + 
        (retirement_date.month() as i32 - dob.month() as i32);
    let retirement_age = total_months as f32 / 12.0;
   
    Retirement{retirement_date,retirement_age,extended_work_month}
}

fn origin_retire_date(dob: NaiveDate, retire_age: u32 )-> NaiveDate{
    let retirement_year = dob.year() + retire_age as i32;
    // Create new date with same month and day
    NaiveDate::from_ymd_opt(
        retirement_year,
        dob.month(),
        dob.day()
    ).unwrap_or(dob)
}

fn extend_every_n_month_by(origin_retire_date: & NaiveDate,policy_start: NaiveDate, n: u32, by: u32, max_extension: u32)-> u32{
    let month_diff = (origin_retire_date.year() - policy_start.year()) * 12 + 
        (origin_retire_date.month() as i32 - policy_start.month() as i32);
    if month_diff < 0 {
        return 0;
    } 
    
    ((month_diff as u32 / n + 1) * by).min(max_extension)
}