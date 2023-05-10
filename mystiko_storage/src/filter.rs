#![forbid(unsafe_code)]

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Condition {
    FILTER(SubFilter),
    AND(Vec<SubFilter>),
    OR(Vec<SubFilter>),
}

#[derive(Clone)]
pub enum ConditionOperator {
    AND,
    OR,
}

#[derive(Clone)]
pub enum Order {
    ASC,
    DESC,
}

#[derive(Clone)]
pub struct OrderBy {
    pub columns: Vec<String>,
    pub order: Order,
}

#[derive(Clone)]
pub struct QueryFilter {
    pub conditions: Vec<Condition>,
    pub conditions_operator: Option<ConditionOperator>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order_by: Option<OrderBy>,
}

#[derive(Default)]
pub struct QueryFilterBuilder {
    conditions: Vec<Condition>,
    conditions_operator: Option<ConditionOperator>,
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
                let wrapped_values: Vec<String> = values.iter().map(|v| format!("'{}'", v)).collect();
                format!("`{}` IN ({})", column, wrapped_values.join(", "))
            }
        }
    }
}

impl Condition {
    pub fn to_sql(&self) -> String {
        match self {
            Condition::FILTER(filter) => filter.to_sql(),
            Condition::AND(filters) => {
                let filters_sql: Vec<String> = filters.iter().map(|f| f.to_sql()).collect();
                filters_sql.join(" AND ")
            }
            Condition::OR(filters) => {
                let filters_sql: Vec<String> = filters.iter().map(|f| f.to_sql()).collect();
                filters_sql.join(" OR ")
            }
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
            let columns_quote: Vec<String> = self.columns.iter().map(|c| format!("`{}`", c)).collect();
            format!("ORDER BY {} {}", columns_quote.join(", "), self.order.as_str())
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
                .map(|c| (c, c.to_sql()))
                .filter(|(_, s)| !s.is_empty())
                .map(|(c, s)| match c {
                    Condition::FILTER(_) => s,
                    _ => {
                        if self.conditions.len() > 1 {
                            format!("({})", s)
                        } else {
                            s
                        }
                    }
                })
                .collect();
            let conditions_operator = match self.conditions_operator.as_ref().unwrap_or(&ConditionOperator::AND) {
                ConditionOperator::AND => "AND",
                ConditionOperator::OR => "OR",
            };
            sqls.push(condition_sqls.join(&format!(" {} ", conditions_operator)));
        }
        if let Some(order_by) = &self.order_by {
            if !order_by.columns.is_empty() {
                sqls.push(order_by.to_sql());
            }
        }
        if let Some(limit) = self.limit {
            sqls.push(format!("LIMIT {}", limit));
            if let Some(offset) = self.offset {
                sqls.push(format!("OFFSET {}", offset));
            }
        }
        sqls.join(" ")
    }
}

impl QueryFilterBuilder {
    pub fn new() -> QueryFilterBuilder {
        QueryFilterBuilder {
            conditions: vec![],
            conditions_operator: None,
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

    pub fn filter_operator(mut self, operator: ConditionOperator) -> Self {
        self.conditions_operator = Some(operator);
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
            conditions_operator: self.conditions_operator,
            limit: self.limit,
            offset: self.offset,
            order_by: self.order_by,
        }
    }
}
