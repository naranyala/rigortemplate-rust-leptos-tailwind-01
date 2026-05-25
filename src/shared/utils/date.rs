use chrono::{DateTime, Utc};

pub fn format_date_short(date: DateTime<Utc>) -> String {
    date.format("%b %d, %Y").to_string()
}

pub fn format_time_ago(date: DateTime<Utc>) -> String {
    format_time_ago_with_now(date, Utc::now())
}

fn format_time_ago_with_now(date: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let duration = now.signed_duration_since(date);
    
    if duration.num_seconds() < 60 {
        "Just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else {
        format!("{}d ago", duration.num_days())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_format_date_short() {
        let date = Utc.with_ymd_and_hms(2024, 5, 26, 0, 0, 0).unwrap();
        assert_eq!(format_date_short(date), "May 26, 2024");
    }

    #[test]
    fn test_format_time_ago_just_now() {
        let now = Utc::now();
        let date = now;
        assert_eq!(format_time_ago_with_now(date, now), "Just now");
    }

    #[test]
    fn test_format_time_ago_minutes() {
        let now = Utc::now();
        let date = now - chrono::Duration::minutes(15);
        assert_eq!(format_time_ago_with_now(date, now), "15m ago");
    }

    #[test]
    fn test_format_time_ago_hours() {
        let now = Utc::now();
        let date = now - chrono::Duration::hours(5);
        assert_eq!(format_time_ago_with_now(date, now), "5h ago");
    }

    #[test]
    fn test_format_time_ago_days() {
        let now = Utc::now();
        let date = now - chrono::Duration::days(10);
        assert_eq!(format_time_ago_with_now(date, now), "10d ago");
    }
}
