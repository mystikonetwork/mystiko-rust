use mystiko_roller::data::calc::calc_rollup_size_array;

#[tokio::test]
pub async fn test_calc_rollup_size_array_0() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![2]),
        (3, vec![2]),
        (4, vec![4]),
        (5, vec![4]),
        (6, vec![4]),
        (7, vec![4]),
        (8, vec![8]),
        (9, vec![8]),
        (10, vec![8]),
        (11, vec![8]),
        (12, vec![8]),
        (13, vec![8]),
        (14, vec![8]),
        (15, vec![8]),
        (16, vec![16]),
        (17, vec![16]),
        (18, vec![16]),
        (19, vec![16]),
        (20, vec![16]),
        (21, vec![16]),
        (22, vec![16]),
        (23, vec![16]),
        (24, vec![16]),
        (25, vec![16]),
        (26, vec![16]),
        (27, vec![16]),
        (28, vec![16]),
        (29, vec![16]),
        (30, vec![16]),
        (31, vec![16]),
        (32, vec![16, 16]),
        (33, vec![16, 16]),
    ];

    for i in (0..32).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_1() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![1]),
        (3, vec![1, 2]),
        (4, vec![1, 2]),
        (5, vec![1, 2]),
        (6, vec![1, 2]),
        (7, vec![1, 2, 4]),
        (8, vec![1, 2, 4]),
        (9, vec![1, 2, 4]),
        (10, vec![1, 2, 4]),
        (11, vec![1, 2, 4]),
        (12, vec![1, 2, 4]),
        (13, vec![1, 2, 4]),
        (14, vec![1, 2, 4]),
        (15, vec![1, 2, 4, 8]),
        (16, vec![1, 2, 4, 8]),
        (17, vec![1, 2, 4, 8]),
        (18, vec![1, 2, 4, 8]),
        (19, vec![1, 2, 4, 8]),
        (20, vec![1, 2, 4, 8]),
        (21, vec![1, 2, 4, 8]),
        (22, vec![1, 2, 4, 8]),
        (23, vec![1, 2, 4, 8]),
        (24, vec![1, 2, 4, 8]),
        (25, vec![1, 2, 4, 8]),
        (26, vec![1, 2, 4, 8]),
        (27, vec![1, 2, 4, 8]),
        (28, vec![1, 2, 4, 8]),
        (29, vec![1, 2, 4, 8]),
        (30, vec![1, 2, 4, 8]),
        (31, vec![1, 2, 4, 8, 16]),
        (32, vec![1, 2, 4, 8, 16]),
        (33, vec![1, 2, 4, 8, 16]),
        (47, vec![1, 2, 4, 8, 16, 16]),
    ];

    for i in (1..33).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_2() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![2]),
        (3, vec![2]),
        (4, vec![2]),
        (5, vec![2]),
        (6, vec![2, 4]),
        (7, vec![2, 4]),
        (8, vec![2, 4]),
        (9, vec![2, 4]),
        (10, vec![2, 4]),
        (11, vec![2, 4]),
        (12, vec![2, 4]),
        (13, vec![2, 4]),
        (14, vec![2, 4, 8]),
        (15, vec![2, 4, 8]),
        (16, vec![2, 4, 8]),
        (17, vec![2, 4, 8]),
        (18, vec![2, 4, 8]),
        (19, vec![2, 4, 8]),
        (20, vec![2, 4, 8]),
        (21, vec![2, 4, 8]),
        (22, vec![2, 4, 8]),
        (23, vec![2, 4, 8]),
        (24, vec![2, 4, 8]),
        (25, vec![2, 4, 8]),
        (26, vec![2, 4, 8]),
        (27, vec![2, 4, 8]),
        (28, vec![2, 4, 8]),
        (29, vec![2, 4, 8]),
        (30, vec![2, 4, 8, 16]),
        (31, vec![2, 4, 8, 16]),
        (32, vec![2, 4, 8, 16]),
        (33, vec![2, 4, 8, 16]),
        (46, vec![2, 4, 8, 16, 16]),
    ];

    for i in (2..34).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_3() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![1]),
        (3, vec![1, 2]),
        (4, vec![1, 2]),
        (5, vec![1, 4]),
        (6, vec![1, 4]),
        (7, vec![1, 4]),
        (8, vec![1, 4]),
        (9, vec![1, 4]),
        (10, vec![1, 4]),
        (11, vec![1, 4]),
        (12, vec![1, 4]),
        (13, vec![1, 4, 8]),
        (14, vec![1, 4, 8]),
        (15, vec![1, 4, 8]),
        (16, vec![1, 4, 8]),
        (17, vec![1, 4, 8]),
        (18, vec![1, 4, 8]),
        (19, vec![1, 4, 8]),
        (20, vec![1, 4, 8]),
        (21, vec![1, 4, 8]),
        (22, vec![1, 4, 8]),
        (23, vec![1, 4, 8]),
        (24, vec![1, 4, 8]),
        (25, vec![1, 4, 8]),
        (26, vec![1, 4, 8]),
        (27, vec![1, 4, 8]),
        (28, vec![1, 4, 8]),
        (29, vec![1, 4, 8, 16]),
        (30, vec![1, 4, 8, 16]),
        (31, vec![1, 4, 8, 16]),
        (32, vec![1, 4, 8, 16]),
        (33, vec![1, 4, 8, 16]),
        (45, vec![1, 4, 8, 16, 16]),
    ];

    for i in (3..35).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_4() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![2]),
        (3, vec![2]),
        (4, vec![4]),
        (5, vec![4]),
        (6, vec![4]),
        (7, vec![4]),
        (8, vec![4]),
        (9, vec![4]),
        (10, vec![4]),
        (11, vec![4]),
        (12, vec![4, 8]),
        (13, vec![4, 8]),
        (14, vec![4, 8]),
        (15, vec![4, 8]),
        (16, vec![4, 8]),
        (17, vec![4, 8]),
        (18, vec![4, 8]),
        (19, vec![4, 8]),
        (20, vec![4, 8]),
        (21, vec![4, 8]),
        (22, vec![4, 8]),
        (23, vec![4, 8]),
        (24, vec![4, 8]),
        (25, vec![4, 8]),
        (26, vec![4, 8]),
        (27, vec![4, 8]),
        (28, vec![4, 8, 16]),
        (29, vec![4, 8, 16]),
        (30, vec![4, 8, 16]),
        (31, vec![4, 8, 16]),
        (32, vec![4, 8, 16]),
        (33, vec![4, 8, 16]),
        (44, vec![4, 8, 16, 16]),
    ];

    for i in (4..36).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_5() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![1]),
        (3, vec![1, 2]),
        (4, vec![1, 2]),
        (5, vec![1, 2]),
        (6, vec![1, 2]),
        (7, vec![1, 2, 4]),
        (8, vec![1, 2, 4]),
        (9, vec![1, 2, 4]),
        (10, vec![1, 2, 4]),
        (11, vec![1, 2, 8]),
        (12, vec![1, 2, 8]),
        (13, vec![1, 2, 8]),
        (14, vec![1, 2, 8]),
        (15, vec![1, 2, 8]),
        (16, vec![1, 2, 8]),
        (17, vec![1, 2, 8]),
        (18, vec![1, 2, 8]),
        (19, vec![1, 2, 8]),
        (20, vec![1, 2, 8]),
        (21, vec![1, 2, 8]),
        (22, vec![1, 2, 8]),
        (23, vec![1, 2, 8]),
        (24, vec![1, 2, 8]),
        (25, vec![1, 2, 8]),
        (26, vec![1, 2, 8]),
        (27, vec![1, 2, 8, 16]),
        (28, vec![1, 2, 8, 16]),
        (29, vec![1, 2, 8, 16]),
        (30, vec![1, 2, 8, 16]),
        (31, vec![1, 2, 8, 16]),
        (32, vec![1, 2, 8, 16]),
        (33, vec![1, 2, 8, 16]),
        (43, vec![1, 2, 8, 16, 16]),
    ];

    for i in (5..37).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_6() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![2]),
        (3, vec![2]),
        (4, vec![2]),
        (5, vec![2]),
        (6, vec![2, 4]),
        (7, vec![2, 4]),
        (8, vec![2, 4]),
        (9, vec![2, 4]),
        (10, vec![2, 8]),
        (11, vec![2, 8]),
        (12, vec![2, 8]),
        (13, vec![2, 8]),
        (14, vec![2, 8]),
        (15, vec![2, 8]),
        (16, vec![2, 8]),
        (17, vec![2, 8]),
        (18, vec![2, 8]),
        (19, vec![2, 8]),
        (20, vec![2, 8]),
        (21, vec![2, 8]),
        (22, vec![2, 8]),
        (23, vec![2, 8]),
        (24, vec![2, 8]),
        (25, vec![2, 8]),
        (26, vec![2, 8, 16]),
        (27, vec![2, 8, 16]),
        (28, vec![2, 8, 16]),
        (29, vec![2, 8, 16]),
        (30, vec![2, 8, 16]),
        (31, vec![2, 8, 16]),
        (32, vec![2, 8, 16]),
        (33, vec![2, 8, 16]),
        (42, vec![2, 8, 16, 16]),
    ];

    for i in (6..38).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_7() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![1]),
        (3, vec![1, 2]),
        (4, vec![1, 2]),
        (5, vec![1, 4]),
        (6, vec![1, 4]),
        (7, vec![1, 4]),
        (8, vec![1, 4]),
        (9, vec![1, 8]),
        (10, vec![1, 8]),
        (11, vec![1, 8]),
        (12, vec![1, 8]),
        (13, vec![1, 8]),
        (14, vec![1, 8]),
        (15, vec![1, 8]),
        (16, vec![1, 8]),
        (17, vec![1, 8]),
        (18, vec![1, 8]),
        (19, vec![1, 8]),
        (20, vec![1, 8]),
        (21, vec![1, 8]),
        (22, vec![1, 8]),
        (23, vec![1, 8]),
        (24, vec![1, 8]),
        (25, vec![1, 8, 16]),
        (26, vec![1, 8, 16]),
        (27, vec![1, 8, 16]),
        (28, vec![1, 8, 16]),
        (29, vec![1, 8, 16]),
        (30, vec![1, 8, 16]),
        (31, vec![1, 8, 16]),
        (32, vec![1, 8, 16]),
        (33, vec![1, 8, 16]),
        (41, vec![1, 8, 16, 16]),
    ];

    for i in (7..39).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_8() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![2]),
        (3, vec![2]),
        (4, vec![4]),
        (5, vec![4]),
        (6, vec![4]),
        (7, vec![4]),
        (8, vec![8]),
        (9, vec![8]),
        (10, vec![8]),
        (11, vec![8]),
        (12, vec![8]),
        (13, vec![8]),
        (14, vec![8]),
        (15, vec![8]),
        (16, vec![8]),
        (17, vec![8]),
        (18, vec![8]),
        (19, vec![8]),
        (20, vec![8]),
        (21, vec![8]),
        (22, vec![8]),
        (23, vec![8]),
        (24, vec![8, 16]),
        (25, vec![8, 16]),
        (26, vec![8, 16]),
        (27, vec![8, 16]),
        (28, vec![8, 16]),
        (29, vec![8, 16]),
        (30, vec![8, 16]),
        (31, vec![8, 16]),
        (32, vec![8, 16]),
        (33, vec![8, 16]),
        (40, vec![8, 16, 16]),
    ];

    for i in (8..40).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_9() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![1]),
        (3, vec![1, 2]),
        (4, vec![1, 2]),
        (5, vec![1, 2]),
        (6, vec![1, 2]),
        (7, vec![1, 2, 4]),
        (8, vec![1, 2, 4]),
        (9, vec![1, 2, 4]),
        (10, vec![1, 2, 4]),
        (11, vec![1, 2, 4]),
        (12, vec![1, 2, 4]),
        (13, vec![1, 2, 4]),
        (14, vec![1, 2, 4]),
        (15, vec![1, 2, 4, 8]),
        (16, vec![1, 2, 4, 8]),
        (17, vec![1, 2, 4, 8]),
        (18, vec![1, 2, 4, 8]),
        (19, vec![1, 2, 4, 8]),
        (20, vec![1, 2, 4, 8]),
        (21, vec![1, 2, 4, 8]),
        (22, vec![1, 2, 4, 8]),
        (23, vec![1, 2, 4, 16]),
        (24, vec![1, 2, 4, 16]),
        (25, vec![1, 2, 4, 16]),
        (39, vec![1, 2, 4, 16, 16]),
    ];

    for i in (9..41).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_10() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![2]),
        (3, vec![2]),
        (4, vec![2]),
        (5, vec![2]),
        (6, vec![2, 4]),
        (7, vec![2, 4]),
        (8, vec![2, 4]),
        (9, vec![2, 4]),
        (10, vec![2, 4]),
        (11, vec![2, 4]),
        (12, vec![2, 4]),
        (13, vec![2, 4]),
        (14, vec![2, 4, 8]),
        (15, vec![2, 4, 8]),
        (16, vec![2, 4, 8]),
        (17, vec![2, 4, 8]),
        (18, vec![2, 4, 8]),
        (19, vec![2, 4, 8]),
        (20, vec![2, 4, 8]),
        (21, vec![2, 4, 8]),
        (22, vec![2, 4, 16]),
        (23, vec![2, 4, 16]),
        (38, vec![2, 4, 16, 16]),
    ];

    for i in (10..42).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_11() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![1]),
        (3, vec![1, 2]),
        (4, vec![1, 2]),
        (5, vec![1, 4]),
        (6, vec![1, 4]),
        (7, vec![1, 4]),
        (8, vec![1, 4]),
        (9, vec![1, 4]),
        (10, vec![1, 4]),
        (11, vec![1, 4]),
        (12, vec![1, 4]),
        (13, vec![1, 4, 8]),
        (14, vec![1, 4, 8]),
        (15, vec![1, 4, 8]),
        (16, vec![1, 4, 8]),
        (17, vec![1, 4, 8]),
        (18, vec![1, 4, 8]),
        (19, vec![1, 4, 8]),
        (20, vec![1, 4, 8]),
        (21, vec![1, 4, 16]),
        (22, vec![1, 4, 16]),
        (23, vec![1, 4, 16]),
        (37, vec![1, 4, 16, 16]),
    ];

    for i in (11..43).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_12() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![2]),
        (3, vec![2]),
        (4, vec![4]),
        (5, vec![4]),
        (6, vec![4]),
        (7, vec![4]),
        (8, vec![4]),
        (9, vec![4]),
        (10, vec![4]),
        (11, vec![4]),
        (12, vec![4, 8]),
        (13, vec![4, 8]),
        (14, vec![4, 8]),
        (15, vec![4, 8]),
        (16, vec![4, 8]),
        (17, vec![4, 8]),
        (18, vec![4, 8]),
        (19, vec![4, 8]),
        (20, vec![4, 16]),
        (21, vec![4, 16]),
        (22, vec![4, 16]),
        (36, vec![4, 16, 16]),
    ];

    for i in (12..44).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_13() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![1]),
        (3, vec![1, 2]),
        (4, vec![1, 2]),
        (5, vec![1, 2]),
        (6, vec![1, 2]),
        (7, vec![1, 2, 4]),
        (8, vec![1, 2, 4]),
        (9, vec![1, 2, 4]),
        (10, vec![1, 2, 4]),
        (11, vec![1, 2, 8]),
        (12, vec![1, 2, 8]),
        (13, vec![1, 2, 8]),
        (14, vec![1, 2, 8]),
        (15, vec![1, 2, 8]),
        (16, vec![1, 2, 8]),
        (17, vec![1, 2, 8]),
        (18, vec![1, 2, 8]),
        (19, vec![1, 2, 16]),
        (20, vec![1, 2, 16]),
        (21, vec![1, 2, 16]),
        (35, vec![1, 2, 16, 16]),
    ];

    for i in (13..45).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_14() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![2]),
        (3, vec![2]),
        (4, vec![2]),
        (5, vec![2]),
        (6, vec![2, 4]),
        (7, vec![2, 4]),
        (8, vec![2, 4]),
        (9, vec![2, 4]),
        (10, vec![2, 8]),
        (11, vec![2, 8]),
        (12, vec![2, 8]),
        (13, vec![2, 8]),
        (14, vec![2, 8]),
        (15, vec![2, 8]),
        (16, vec![2, 8]),
        (17, vec![2, 8]),
        (18, vec![2, 16]),
        (19, vec![2, 16]),
        (20, vec![2, 16]),
        (34, vec![2, 16, 16]),
    ];

    for i in (14..46).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}

#[tokio::test]
pub async fn test_calc_rollup_size_array_15() {
    let test_cases = vec![
        (0, vec![]),
        (1, vec![1]),
        (2, vec![1]),
        (3, vec![1, 2]),
        (4, vec![1, 2]),
        (5, vec![1, 4]),
        (6, vec![1, 4]),
        (7, vec![1, 4]),
        (8, vec![1, 4]),
        (9, vec![1, 8]),
        (10, vec![1, 8]),
        (11, vec![1, 8]),
        (12, vec![1, 8]),
        (13, vec![1, 8]),
        (14, vec![1, 8]),
        (15, vec![1, 8]),
        (16, vec![1, 8]),
        (17, vec![1, 16]),
        (18, vec![1, 16]),
        (19, vec![1, 16]),
        (33, vec![1, 16, 16]),
    ];

    for i in (15..47).step_by(16) {
        for (queued, expected) in test_cases.clone() {
            let result = calc_rollup_size_array(i, queued);
            assert_eq!(result, expected, "Test failed for queued: {}", queued);
        }
    }
}
