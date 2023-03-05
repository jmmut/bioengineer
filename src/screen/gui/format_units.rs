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
    if unsigned_quantity < KILO {
        format!("{} {}", round_with_some_decimals(quantity), unit_name)
    } else if unsigned_quantity < MEGA {
        format!("{} K{}", round_with_some_decimals(quantity / KILO), unit_name)
    } else if unsigned_quantity < GIGA {
        format!("{} M{}", round_with_some_decimals(quantity / MEGA), unit_name)
    } else if unsigned_quantity < TERA {
        format!("{} G{}", round_with_some_decimals(quantity / GIGA), unit_name)
    } else if unsigned_quantity < PETA {
        format!("{} T{}", round_with_some_decimals(quantity / TERA), unit_name)
    } else if unsigned_quantity < EXA {
        format!("{} P{}", round_with_some_decimals(quantity / PETA), unit_name)
    } else if unsigned_quantity < ZETTA {
        format!("{} E{}", round_with_some_decimals(quantity / EXA), unit_name)
    } else if unsigned_quantity < YOTTA {
        format!("{} Z{}", round_with_some_decimals(quantity / ZETTA), unit_name)
    } else {
        format!("{} Y{}", round_with_some_decimals(quantity / YOTTA), unit_name)
    }
}

fn round_with_some_decimals(quantity: f64) -> f64 {
    let unsigned_quantity = quantity.abs();
    if unsigned_quantity >= 100.0 {
        quantity.floor()
    } else if unsigned_quantity >= 10.0 {
        (quantity * 10.0).floor() / 10.0
    } else {
        (quantity * 100.0).floor() / 100.0
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
        assert_eq!(format_unit(1000.0, " paperclips"), "1 K paperclips");
        assert_eq!(format_unit(0.0, "W"), "0 W");
        assert_eq!(format_unit(0.5, "W"), "0.5 W");
        assert_eq!(format_unit(-0.5, "W"), "-0.5 W");
        assert_eq!(format_unit(10.0, "W"), "10 W");
        assert_eq!(format_unit(999.0, "W"), "999 W");
        assert_eq!(format_unit(1000.0, "W"), "1 KW");
        assert_eq!(format_unit(1110.0, "W"), "1.11 KW");
        assert_eq!(format_unit(10000.0, "W"), "10 KW");
        assert_eq!(format_unit(10100.0, "W"), "10.1 KW");
        assert_eq!(format_unit(100100.0, "W"), "100 KW");
        assert_eq!(format_unit(999999.0, "W"), "999 KW");
        assert_eq!(format_unit(1000000.0, "W"), "1 MW");
        assert_eq!(format_unit(999999999.0, "W"), "999 MW");
        assert_eq!(format_unit(1000000000.0, "W"), "1 GW");
        assert_eq!(format_unit(999999999999.0, "W"), "999 GW");
        assert_eq!(format_unit(1000000000000.0, "W"), "1 TW");
        assert_eq!(format_unit(999999999999999.0, "W"), "999 TW");
        assert_eq!(format_unit(1000000000000000.0, "W"), "1 PW");
        assert_eq!(format_unit(999999999999999935.0, "W"), "999 PW");
        assert_eq!(format_unit(1000000000000000000.0, "W"), "1 EW");
        assert_eq!(format_unit(999999999999999934000.0, "W"), "999 EW");
        assert_eq!(format_unit(1000000000000000000000.0, "W"), "1 ZW");
        assert_eq!(format_unit(999999999999999916000000.0, "W"), "999 ZW");
        assert_eq!(format_unit(1000000000000000000000000.0, "W"), "1 YW");
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
