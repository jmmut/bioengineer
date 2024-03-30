const KILO: f64 = 1.0e3;
const MEGA: f64 = 1.0e6;
const GIGA: f64 = 1.0e9;
const TERA: f64 = 1.0e12;
const PETA: f64 = 1.0e15;
const EXA: f64 = 1.0e18;
const ZETTA: f64 = 1.0e21;
const YOTTA: f64 = 1.0e24;

#[rustfmt::skip]
pub fn format_unit(quantity: f64, unit_name: &str) -> String {
    let unsigned_quantity = quantity.abs().floor();
    fn custom_format(quantity:f64, scale: f64, scale_str: &str, unit_name: &str) -> String {
        let rounded_quantity = round_with_some_decimals(quantity / scale);
        let precision = precision(quantity / scale);
        format!("{rounded_quantity:.precision$} {scale_str}{unit_name}")
    }
    if unsigned_quantity < KILO {
        custom_format(quantity, 1.0, "", unit_name)
    } else if unsigned_quantity < MEGA {
        custom_format(quantity, KILO, "K", unit_name)
    } else if unsigned_quantity < GIGA {
        custom_format(quantity, MEGA, "M", unit_name)
    } else if unsigned_quantity < TERA {
        custom_format(quantity, GIGA, "G", unit_name)
    } else if unsigned_quantity < PETA {
        custom_format(quantity, TERA, "T", unit_name)
    } else if unsigned_quantity < EXA {
        custom_format(quantity, PETA, "P", unit_name)
    } else if unsigned_quantity < ZETTA {
        custom_format(quantity, EXA, "E", unit_name)
    } else if unsigned_quantity < YOTTA {
        custom_format(quantity, ZETTA, "Z", unit_name)
    } else {
        custom_format(quantity, YOTTA, "Y", unit_name)
    }
}

fn round_with_some_decimals(quantity: f64) -> f64 {
    let precision = precision(quantity);
    let precision_factor = 10.0f64.powi(precision as i32);
    (quantity * precision_factor).trunc() / precision_factor
}

fn precision(quantity: f64) -> usize {
    let unsigned_quantity = quantity.abs();
    if unsigned_quantity >= 100.0 {
        0
    } else if unsigned_quantity >= 10.0 {
        1
    } else {
        2
    }
}

pub fn format_age(age_in_minutes: i64) -> String {
    const MINUTES_PER_HOUR: i64 = 60;
    const HOURS_PER_DAY: i64 = 24;
    const DAYS_PER_YEAR: i64 = 365;
    const HOURS_PER_YEAR: i64 = HOURS_PER_DAY * DAYS_PER_YEAR;

    if age_in_minutes < MINUTES_PER_HOUR {
        format!("{}", format_time(age_in_minutes, "minute"))
    } else if age_in_minutes < MINUTES_PER_HOUR * HOURS_PER_DAY {
        format!(
            "{}, {}",
            format_time(age_in_minutes / MINUTES_PER_HOUR, "hour"),
            format_time(age_in_minutes % MINUTES_PER_HOUR, "minute")
        )
    } else if age_in_minutes < MINUTES_PER_HOUR * HOURS_PER_DAY * DAYS_PER_YEAR {
        format!(
            "{}, {}, {}",
            format_time(age_in_minutes / MINUTES_PER_HOUR / HOURS_PER_DAY, "day"),
            format_time(age_in_minutes / MINUTES_PER_HOUR % HOURS_PER_DAY, "hour"),
            format_time(age_in_minutes % MINUTES_PER_HOUR, "minute")
        )
    } else {
        format!(
            "{}, {}, {}, {}",
            format_time(age_in_minutes / MINUTES_PER_HOUR / HOURS_PER_YEAR, "year"),
            format_time(
                age_in_minutes / MINUTES_PER_HOUR / HOURS_PER_DAY % DAYS_PER_YEAR,
                "day"
            ),
            format_time(age_in_minutes / MINUTES_PER_HOUR % HOURS_PER_DAY, "hour"),
            format_time(age_in_minutes % MINUTES_PER_HOUR, "minute")
        )
    }
}

fn format_time(value: i64, unit: &str) -> String {
    format!("{} {}{}", value, unit, single_str(value))
}

fn single_str(number: i64) -> String {
    if number == 1 {
        "".to_string()
    } else {
        "s".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_units() {
        assert_eq!(format_unit(1000.0, " paperclips"), "1.00 K paperclips");
        assert_eq!(format_unit(0.0, "W"), "0.00 W");
        assert_eq!(format_unit(0.5, "W"), "0.50 W");
        assert_eq!(format_unit(-0.5, "W"), "-0.50 W");
        assert_eq!(format_unit(10.0, "W"), "10.0 W");
        assert_eq!(format_unit(999.0, "W"), "999 W");
        assert_eq!(format_unit(1000.0, "W"), "1.00 KW");
        assert_eq!(format_unit(1110.0, "W"), "1.11 KW");
        assert_eq!(format_unit(-1110.0, "W"), "-1.11 KW");
        assert_eq!(format_unit(10000.0, "W"), "10.0 KW");
        assert_eq!(format_unit(10100.0, "W"), "10.1 KW");
        assert_eq!(format_unit(100100.0, "W"), "100 KW");
        assert_eq!(format_unit(999999.0, "W"), "999 KW");
        assert_eq!(format_unit(1000000.0, "W"), "1.00 MW");
        assert_eq!(format_unit(999999999.0, "W"), "999 MW");
        assert_eq!(format_unit(1000000000.0, "W"), "1.00 GW");
        assert_eq!(format_unit(999999999999.0, "W"), "999 GW");
        assert_eq!(format_unit(1000000000000.0, "W"), "1.00 TW");
        assert_eq!(format_unit(999999999999999.0, "W"), "999 TW");
        assert_eq!(format_unit(1000000000000000.0, "W"), "1.00 PW");
        assert_eq!(format_unit(999999999999999935.0, "W"), "999 PW");
        assert_eq!(format_unit(1000000000000000000.0, "W"), "1.00 EW");
        assert_eq!(format_unit(999999999999999934000.0, "W"), "999 EW");
        assert_eq!(format_unit(1000000000000000000000.0, "W"), "1.00 ZW");
        assert_eq!(format_unit(999999999999999916000000.0, "W"), "999 ZW");
        assert_eq!(format_unit(1000000000000000000000000.0, "W"), "1.00 YW");
        assert_eq!(format_unit(1000000000000000000000000000.0, "W"), "1000 YW");
    }

    #[test]
    fn test_format_age() {
        assert_eq!(format_age(0), "0 minutes");
        assert_eq!(format_age(1), "1 minute");
        assert_eq!(format_age(10), "10 minutes");
        assert_eq!(format_age((1) * 60 + 0), "1 hour, 0 minutes");
        assert_eq!(format_age((17) * 60 + 0), "17 hours, 0 minutes");
        assert_eq!(format_age((26) * 60 + 0), "1 day, 2 hours, 0 minutes");
        assert_eq!(
            format_age((24 * 3 + 17) * 60 + 0),
            "3 days, 17 hours, 0 minutes"
        );
        assert_eq!(
            format_age((365 * 24 + 3 * 24 + 1) * 60 + 0),
            "1 year, 3 days, 1 hour, 0 minutes"
        );
        assert_eq!(
            format_age((1000000 * 365 * 24 + 24 + 10) * 60 + 0),
            "1000000 years, 1 day, 10 hours, 0 minutes"
        );
    }
}
