#[cfg(test)]
mod sampling {
    use itertools::Itertools;
    use rinex::prelude::*;
    use rinex::preprocessing::*;
    use std::str::FromStr;
    #[test]
    fn nav() {
        let path = env!("CARGO_MANIFEST_DIR").to_owned()
            + "/../test_resources/NAV/V3/AMEL00NLD_R_20210010000_01D_MN.rnx";
        let rinex = Rinex::from_file(&path).unwrap();

        assert!(
            rinex.sampling_histogram().sorted().eq(vec![
                (Duration::from_seconds(15.0 * 60.0), 1),
                (Duration::from_seconds(25.0 * 60.0), 1),
                (Duration::from_seconds(4.0 * 3600.0 + 45.0 * 60.0), 2),
                (Duration::from_seconds(5.0 * 3600.0 + 30.0 * 60.0), 1),
            ]),
            "sampling_histogram failed"
        );

        let initial_len = rinex.epoch().count();
        let decimated = rinex.decimate_by_interval(Duration::from_seconds(10.0));
        assert!(
            initial_len == decimated.epoch().count(),
            "decim with too small time interval failed"
        );
        let decimated = decimated.decimate_by_interval(Duration::from_hours(1.0));
        assert!(
            initial_len == decimated.epoch().count() + 2,
            "failed to decimate to 1 hour epoch interval"
        );

        let decimated = rinex.decimate_by_ratio(2);
        assert_eq!(decimated.epoch().count(), 3, "decim by 2 failed");

        let decimated = decimated.decimate_by_ratio(2);
        assert!(decimated.epoch().count() == 2, "decim by 2 + 2 failed");
    }
    #[test]
    fn meteo() {
        let path = env!("CARGO_MANIFEST_DIR").to_owned() + "/../test_resources/MET/V2/abvi0010.15m";
        let rinex = Rinex::from_file(&path).unwrap();
        assert_eq!(
            rinex.dominant_sample_rate(),
            Some(Duration::from_seconds(60.0)),
        );
        assert!(
            rinex.data_gaps(None).eq(vec![
                (
                    Epoch::from_str("2015-01-01T00:09:00 UTC").unwrap(),
                    Duration::from_parts(0, 31860000000000),
                ),
                (
                    Epoch::from_str("2015-01-01T09:04:00 UTC").unwrap(),
                    Duration::from_parts(0, 37260000000000),
                ),
                (
                    Epoch::from_str("2015-01-01T19:54:00 UTC").unwrap(),
                    Duration::from_parts(0, 10860000000000),
                ),
                (
                    Epoch::from_str("2015-01-01T23:02:00 UTC").unwrap(),
                    Duration::from_parts(0, 420000000000),
                ),
                (
                    Epoch::from_str("2015-01-01T23:21:00 UTC").unwrap(),
                    Duration::from_parts(0, 1860000000000),
                ),
            ]),
            "data_gaps(tol=None) failed",
        );
        assert!(
            rinex
                .data_gaps(Some(Duration::from_seconds(10.0 * 60.0)))
                .eq(vec![
                    (
                        Epoch::from_str("2015-01-01T00:09:00 UTC").unwrap(),
                        Duration::from_parts(0, 31860000000000),
                    ),
                    (
                        Epoch::from_str("2015-01-01T09:04:00 UTC").unwrap(),
                        Duration::from_parts(0, 37260000000000),
                    ),
                    (
                        Epoch::from_str("2015-01-01T19:54:00 UTC").unwrap(),
                        Duration::from_parts(0, 10860000000000),
                    ),
                    (
                        Epoch::from_str("2015-01-01T23:21:00 UTC").unwrap(),
                        Duration::from_parts(0, 1860000000000),
                    ),
                ]),
            "data_gaps(tol=10') failed",
        );
        assert!(
            rinex
                .data_gaps(Some(Duration::from_seconds(3.0 * 3600.0)))
                .eq(vec![
                    (
                        Epoch::from_str("2015-01-01T00:09:00 UTC").unwrap(),
                        Duration::from_parts(0, 31860000000000),
                    ),
                    (
                        Epoch::from_str("2015-01-01T09:04:00 UTC").unwrap(),
                        Duration::from_parts(0, 37260000000000),
                    ),
                    (
                        Epoch::from_str("2015-01-01T19:54:00 UTC").unwrap(),
                        Duration::from_parts(0, 10860000000000),
                    ),
                ]),
            "data_gaps(tol=3h) failed",
        );
    }
}
