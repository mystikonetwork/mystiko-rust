pub enum SubFilter {
    IsNull { column: String },
    IsNotNull { column: String },
    Equal { column: String, value: String },
    NotEqual { column: String, value: String },
    Greater { column: String, value: String },
    GreaterEqual { column: String, value: String },
    Less { column: String, value: String },
    LessEqual { column: String, value: String },
    BetweenAnd { column: String, values: [String; 2] },
    IN { column: String, values: Vec<String> },
}

pub enum Condition {
    FILTER { filter: SubFilter },
    AND { left: SubFilter, right: SubFilter },
    OR { left: SubFilter, right: SubFilter },
}

pub enum Order {
    ASC,
    DESC,
}

pub struct OrderBy {
    columns: Vec<String>,
    order: Order,
}

pub struct QueryFilter {
    conditions: Vec<Condition>,
    limit: Option<u64>,
    offset: Option<u64>,
    order_by: Option<OrderBy>,
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
            SubFilter::IsNull { column } => format!("`{}` IS NULL", column),
            SubFilter::IsNotNull { column } => format!("`{}` IS NOT NULL", column),
            SubFilter::Equal { column, value } => format!("`{}` = '{}'", column, value),
            SubFilter::NotEqual { column, value } => format!("`{}` != '{}'", column, value),
            SubFilter::Greater { column, value } => format!("`{}` > '{}'", column, value),
            SubFilter::GreaterEqual { column, value } => format!("`{}` >= '{}'", column, value),
            SubFilter::Less { column, value } => format!("`{}` < '{}'", column, value),
            SubFilter::LessEqual { column, value } => format!("`{}` <= '{}'", column, value),
            SubFilter::BetweenAnd { column, values } => {
                format!("`{}` BETWEEN '{}' AND '{}'", column, values[0], values[1])
            }
            SubFilter::IN { column, values } => {
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
            Condition::FILTER { filter } => filter.to_sql(),
            Condition::AND { left, right } => format!("{} AND {}", left.to_sql(), right.to_sql()),
            Condition::OR { left, right } => format!("{} OR {}", left.to_sql(), right.to_sql()),
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

impl QueryFilter {
    pub fn to_sql(&self) -> String {
        let mut sqls: Vec<String> = Vec::new();
        if !self.conditions.is_empty() {
            let condition_sqls: Vec<String> = self.conditions.iter().map(|c| c.to_sql()).collect();
            sqls.push(condition_sqls.join(" AND "));
        }
        if let Some(limit) = self.limit {
            sqls.push(format!("LIMIT {}", limit));
        }
        if let Some(offset) = self.offset {
            sqls.push(format!("OFFSET {}", offset));
        }
        if let Some(OrderBy { columns, order }) = &self.order_by {
            if !columns.is_empty() {
                let columns_quote: Vec<String> =
                    columns.iter().map(|c| format!("`{}`", c)).collect();
                sqls.push(format!(
                    "ORDER BY {} {}",
                    columns_quote.join(", "),
                    order.as_str()
                ));
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
