pub enum SubFilter {
    IsNull(String),
    IsNotNull(String),
    Equal(String, String),
    NotEqual(String, String),
    Greater(String, String),
    GreaterEqual(String, String),
    Less(String, String),
    LessEqual(String, String),
    BetweenAnd(String, [String; 2]),
    IN(String, Vec<String>),
}

pub enum Condition {
    FILTER(SubFilter),
    AND(SubFilter, SubFilter),
    OR(SubFilter, SubFilter),
}

pub enum Order {
    ASC,
    DESC,
}

pub struct OrderBy {
    pub columns: Vec<String>,
    pub order: Order,
}

pub struct QueryFilter {
    pub conditions: Vec<Condition>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order_by: Option<OrderBy>,
}

pub struct QueryFilterBuilder {
    conditions: Vec<Condition>,
    limit: Option<u64>,
    offset: Option<u64>,
    order_by: Option<OrderBy>,
}

impl SubFilter {
    pub fn to_sql(&self) -> String {
        match self {
            SubFilter::IsNull(column) => format!("`{}` IS NULL", column),
            SubFilter::IsNotNull(column) => format!("`{}` IS NOT NULL", column),
            SubFilter::Equal(column, value) => format!("`{}` = '{}'", column, value),
            SubFilter::NotEqual(column, value) => format!("`{}` != '{}'", column, value),
            SubFilter::Greater(column, value) => format!("`{}` > '{}'", column, value),
            SubFilter::GreaterEqual(column, value) => format!("`{}` >= '{}'", column, value),
            SubFilter::Less(column, value) => format!("`{}` < '{}'", column, value),
            SubFilter::LessEqual(column, value) => format!("`{}` <= '{}'", column, value),
            SubFilter::BetweenAnd(column, values) => {
                format!("`{}` BETWEEN '{}' AND '{}'", column, values[0], values[1])
            }
            SubFilter::IN(column, values) => {
                let wrapped_values: Vec<String> =
                    values.iter().map(|v| format!("'{}'", v)).collect();
                format!("`{}` IN [{}]", column, wrapped_values.join(", "))
            }
        }
    }
}

impl Condition {
    pub fn to_sql(&self) -> String {
        match self {
            Condition::FILTER(filter) => filter.to_sql(),
            Condition::AND(left, right) => format!("{} AND {}", left.to_sql(), right.to_sql()),
            Condition::OR(left, right) => format!("{} OR {}", left.to_sql(), right.to_sql()),
        }
    }
}

impl Order {
    pub fn as_str(&self) -> &str {
        match self {
            Order::ASC => "ASC",
            Order::DESC => "DESC",
        }
    }
}

impl OrderBy {
    pub fn to_sql(&self) -> String {
        if !self.columns.is_empty() {
            let columns_quote: Vec<String> =
                self.columns.iter().map(|c| format!("`{}`", c)).collect();
            format!(
                "ORDER BY {} {}",
                columns_quote.join(", "),
                self.order.as_str()
            )
        } else {
            String::new()
        }
    }
}

impl QueryFilter {
    pub fn to_sql(&self) -> String {
        let mut sqls: Vec<String> = Vec::new();
        if !self.conditions.is_empty() {
            let condition_sqls: Vec<String> = self
                .conditions
                .iter()
                .map(|c| match c {
                    Condition::FILTER(_) => c.to_sql(),
                    _ => {
                        if self.conditions.len() > 1 {
                            format!("({})", c.to_sql())
                        } else {
                            c.to_sql()
                        }
                    }
                })
                .collect();
            sqls.push(condition_sqls.join(" AND "));
        }
        if let Some(limit) = self.limit {
            sqls.push(format!("LIMIT {}", limit));
        }
        if let Some(offset) = self.offset {
            sqls.push(format!("OFFSET {}", offset));
        }
        if let Some(order_by) = &self.order_by {
            if !order_by.columns.is_empty() {
                sqls.push(order_by.to_sql());
            }
        }
        sqls.join(" ")
    }
}

impl QueryFilterBuilder {
    pub fn new() -> QueryFilterBuilder {
        QueryFilterBuilder {
            conditions: vec![],
            limit: None,
            offset: None,
            order_by: None,
        }
    }

    pub fn filter(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn filters(mut self, conditions: Vec<Condition>) -> Self {
        self.conditions.extend(conditions);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn order_by(mut self, columns: Vec<String>, order: Order) -> Self {
        self.order_by = Some(OrderBy { columns, order });
        self
    }

    pub fn build(self) -> QueryFilter {
        QueryFilter {
            conditions: self.conditions,
            limit: self.limit,
            offset: self.offset,
            order_by: self.order_by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::Order::{ASC, DESC};

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
            .order_by(vec![String::from("c4")], DESC)
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
            .order_by(vec![String::from("c15")], ASC)
            .build();
        assert_eq!(
            qf8.to_sql(),
            "\
            (`c10` = 'v10' OR `c11` IN ['v11', 'v12']) \
            AND (`c12` BETWEEN 'v13' AND 'v14' AND `c13` >= 'v15') \
            LIMIT 30 OFFSET 40 ORDER BY `c15` ASC"
        );
    }
}
