use mystiko_storage::filter::*;

#[test]
fn test_sub_filter() {
    let sf1 = SubFilter::IsNull(String::from("c1"));
    assert_eq!(sf1.to_sql(), "`c1` IS NULL");
    let sf2 = SubFilter::IsNotNull(String::from("c2"));
    assert_eq!(sf2.to_sql(), "`c2` IS NOT NULL");
    let sf3 = SubFilter::Equal(String::from("c3"), String::from("v3"));
    assert_eq!(sf3.to_sql(), "`c3` = 'v3'");
    let sf4 = SubFilter::NotEqual(String::from("c4"), String::from("v4"));
    assert_eq!(sf4.to_sql(), "`c4` != 'v4'");
    let sf5 = SubFilter::Greater(String::from("c5"), String::from("v5"));
    assert_eq!(sf5.to_sql(), "`c5` > 'v5'");
    let sf6 = SubFilter::GreaterEqual(String::from("c6"), String::from("v6"));
    assert_eq!(sf6.to_sql(), "`c6` >= 'v6'");
    let sf7 = SubFilter::Less(String::from("c7"), String::from("v7"));
    assert_eq!(sf7.to_sql(), "`c7` < 'v7'");
    let sf8 = SubFilter::LessEqual(String::from("c8"), String::from("v8"));
    assert_eq!(sf8.to_sql(), "`c8` <= 'v8'");
    let sf9 = SubFilter::BetweenAnd(
        String::from("c9"),
        [String::from("v10"), String::from("v11")],
    );
    assert_eq!(sf9.to_sql(), "`c9` BETWEEN 'v10' AND 'v11'");
    let sf10 = SubFilter::IN(
        String::from("c10"),
        vec![
            String::from("v12"),
            String::from("v13"),
            String::from("v14"),
        ],
    );
    assert_eq!(sf10.to_sql(), "`c10` IN ['v12', 'v13', 'v14']");
}

#[test]
fn test_condition() {
    let c1 = Condition::FILTER(SubFilter::Equal(String::from("c1"), String::from("v1")));
    assert_eq!(c1.to_sql(), "`c1` = 'v1'");
    let c2 = Condition::AND(
        SubFilter::Equal(String::from("c2"), String::from("v2")),
        SubFilter::IsNull(String::from("c3")),
    );
    assert_eq!(c2.to_sql(), "`c2` = 'v2' AND `c3` IS NULL");
    let c3 = Condition::OR(
        SubFilter::NotEqual(String::from("c3"), String::from("v3")),
        SubFilter::IsNotNull(String::from("c4")),
    );
    assert_eq!(c3.to_sql(), "`c3` != 'v3' OR `c4` IS NOT NULL");
}

#[test]
fn test_order_by() {
    let o1: OrderBy = OrderBy {
        columns: vec![],
        order: Order::ASC,
    };
    assert_eq!(o1.to_sql(), "");
    let o2: OrderBy = OrderBy {
        columns: vec![String::from("c1"), String::from("c2")],
        order: Order::ASC,
    };
    assert_eq!(o2.to_sql(), "ORDER BY `c1`, `c2` ASC");
    let o3: OrderBy = OrderBy {
        columns: vec![String::from("c3")],
        order: Order::DESC,
    };
    assert_eq!(o3.to_sql(), "ORDER BY `c3` DESC");
}

#[test]
fn test_query_filter() {
    let qf1 = QueryFilterBuilder::new().filters(vec![]).build();
    assert_eq!(qf1.to_sql(), "");
    let qf2 = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::IsNull(String::from("c1"))))
        .build();
    assert_eq!(qf2.to_sql(), "`c1` IS NULL");
    let qf3 = QueryFilterBuilder::new()
        .filters(vec![Condition::FILTER(SubFilter::Equal(
            String::from("c2"),
            String::from("v2"),
        ))])
        .limit(30)
        .build();
    assert_eq!(qf3.to_sql(), "`c2` = 'v2' LIMIT 30");
    let qf4 = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::Greater(
            String::from("c3"),
            String::from("v3"),
        )))
        .order_by(vec![String::from("c4")], Order::DESC)
        .build();
    assert_eq!(qf4.to_sql(), "`c3` > 'v3' ORDER BY `c4` DESC");
    let qf5 = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::IsNull(String::from("c4"))))
        .offset(12)
        .build();
    assert_eq!(qf5.to_sql(), "`c4` IS NULL OFFSET 12");
    let qf6 = QueryFilterBuilder::new()
        .filter(Condition::AND(
            SubFilter::IsNull(String::from("c5")),
            SubFilter::Equal(String::from("c6"), String::from("v6")),
        ))
        .build();
    assert_eq!(qf6.to_sql(), "`c5` IS NULL AND `c6` = 'v6'");
    let qf7 = QueryFilterBuilder::new()
        .filters(vec![
            Condition::FILTER(SubFilter::Equal(String::from("c7"), String::from("v7"))),
            Condition::OR(
                SubFilter::IsNotNull(String::from("c8")),
                SubFilter::Less(String::from("c9"), String::from("v9")),
            ),
        ])
        .build();
    assert_eq!(
        qf7.to_sql(),
        "`c7` = 'v7' AND (`c8` IS NOT NULL OR `c9` < 'v9')"
    );
    let qf8 = QueryFilterBuilder::new()
        .filters(vec![
            Condition::OR(
                SubFilter::Equal(String::from("c10"), String::from("v10")),
                SubFilter::IN(
                    String::from("c11"),
                    vec![String::from("v11"), String::from("v12")],
                ),
            ),
            Condition::AND(
                SubFilter::BetweenAnd(
                    String::from("c12"),
                    [String::from("v13"), String::from("v14")],
                ),
                SubFilter::GreaterEqual(String::from("c13"), String::from("v15")),
            ),
        ])
        .limit(30)
        .offset(40)
        .order_by(vec![String::from("c15")], Order::ASC)
        .build();
    assert_eq!(
        qf8.to_sql(),
        "\
            (`c10` = 'v10' OR `c11` IN ['v11', 'v12']) \
            AND (`c12` BETWEEN 'v13' AND 'v14' AND `c13` >= 'v15') \
            ORDER BY `c15` ASC LIMIT 30 OFFSET 40"
    );
}
