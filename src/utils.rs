//  Million, Billion, Trillion
const PARAM_UNITS: &'static [&'static str] = &["", "K", "M", "B", "T"];

pub fn pretty_floating_point(value: f64) -> String {
    if value == value.floor() {
        // 少数第一位切り捨てても同じ場合
        format!("{:.0}", value)
    } else {
        format!("{:.1}", (value * 100.).ceil() / 100.)
    }
}

/// Prettify the parameter size
pub fn pretty_param_size(param_size: i64) -> (String, &'static str) {
    let mut param_size = param_size as f64;
    for (_i, unit) in PARAM_UNITS.iter().enumerate() {
        if param_size < 1024. / 10. {
            // 少数第一位まで表示。小数点以下が0の場合は表示しない
            return (pretty_floating_point(param_size), unit);
        } else {
            param_size /= 1024.;
        }
    }

    (
        pretty_floating_point(param_size),
        PARAM_UNITS.last().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pretty_param_size() {
        assert_eq!(pretty_param_size(0), ("0".to_string(), ""));
        assert_eq!(pretty_param_size(1), ("1".to_string(), ""));
        assert_eq!(pretty_param_size(500), ("0.5".to_string(), "K"));
        assert_eq!(pretty_param_size(1023), ("1.0".to_string(), "K"));
        assert_eq!(pretty_param_size(1024), ("1".to_string(), "K"));
        assert_eq!(pretty_param_size(1024 * 1024), ("1".to_string(), "M"));
        assert_eq!(
            pretty_param_size((3.5 * 1024. * 1024.) as i64),
            ("3.5".to_string(), "M")
        );
        assert_eq!(pretty_param_size(2 * 1024 * 1024), ("2".to_string(), "M"));
        assert_eq!(
            pretty_param_size(3 * 1024 * 1024 * 1024),
            ("3".to_string(), "B")
        );
        assert_eq!(
            pretty_param_size(4 * 1024 * 1024 * 1024 * 1024),
            ("4".to_string(), "T")
        );
        assert_eq!(
            pretty_param_size(5 * 1024 * 1024 * 1024 * 1024 * 1024),
            ("5".to_string(), "T")
        );
    }
}
