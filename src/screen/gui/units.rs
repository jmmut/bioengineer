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
}
