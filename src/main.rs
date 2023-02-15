use chrono::{Datelike, Local, NaiveDate, Weekday};

fn main() {
    // LDOC is April 26, 2023
    let ldoc = chrono::NaiveDate::from_ymd_opt(2023, 4, 26).unwrap();

    // Spring break is from March 11 to March 19, 2023
    let start_break = NaiveDate::from_ymd_opt(2023, 3, 11).unwrap();
    let end_break = NaiveDate::from_ymd_opt(2023, 3, 19).unwrap();

    // Get today's date
    let today = Local::now().naive_local().date();

    // Calculate the number of days and weekdays until LDOC, excluding spring break from weekdays
    let mut days_until_ldoc = 0;
    let mut weekdays_until_ldoc = 0;
    let mut date = today;
    while date < ldoc {
        days_until_ldoc += 1;
        if date.weekday() != Weekday::Sat && date.weekday() != Weekday::Sun {
            if date < start_break || date > end_break {
                weekdays_until_ldoc += 1;
            }
        }
        date += chrono::Duration::days(1);
    }

    println!(
        "There are {} days ({} weekdays) until LDOC.\nHang in there Blue Devils!",
        days_until_ldoc, weekdays_until_ldoc
    );
}
