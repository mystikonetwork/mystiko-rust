use crate::column::ColumnValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubFilterOperator {
    IsNull,
    IsNotNull,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    BetweenAnd,
    In,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionOperator {
    AND,
    OR,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Order {
    ASC,
    DESC,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubFilter {
    pub operator: SubFilterOperator,
    pub column: String,
    pub values: Vec<ColumnValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    pub operator: ConditionOperator,
    pub sub_filters: Vec<SubFilter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderBy {
    pub columns: Vec<String>,
    pub order: Order,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryFilter {
    pub conditions: Vec<Condition>,
    pub conditions_operator: ConditionOperator,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order_by: Option<OrderBy>,
}

#[derive(Default, Debug, Clone)]
pub struct QueryFilterBuilder {
    conditions: Vec<Condition>,
    conditions_operator: Option<ConditionOperator>,
    limit: Option<u64>,
    offset: Option<u64>,
    order_by: Option<OrderBy>,
}

impl SubFilter {
    pub fn is_null<C: ToString>(column: C) -> Self {
        Self {
            operator: SubFilterOperator::IsNull,
            column: column.to_string(),
            values: vec![],
        }
    }
    pub fn is_not_null<C: ToString>(column: C) -> Self {
        Self {
            operator: SubFilterOperator::IsNotNull,
            column: column.to_string(),
            values: vec![],
        }
    }
    pub fn equal<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::Equal,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }
    pub fn not_equal<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::NotEqual,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }
    pub fn greater<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::Greater,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }
    pub fn greater_equal<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::GreaterEqual,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }
    pub fn less<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::Less,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }
    pub fn less_equal<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::LessEqual,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }
    pub fn between_and<C: ToString, V: Into<ColumnValue>>(column: C, value1: V, value2: V) -> Self {
        Self {
            operator: SubFilterOperator::BetweenAnd,
            column: column.to_string(),
            values: vec![value1.into(), value2.into()],
        }
    }
    pub fn in_list<C: ToString, V: Into<ColumnValue>>(column: C, values: Vec<V>) -> Self {
        Self {
            operator: SubFilterOperator::In,
            column: column.to_string(),
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl Condition {
    pub fn filter(sub_filter: SubFilter) -> Self {
        Self {
            operator: ConditionOperator::AND,
            sub_filters: vec![sub_filter],
        }
    }

    pub fn and(sub_filters: Vec<SubFilter>) -> Self {
        Self {
            operator: ConditionOperator::AND,
            sub_filters,
        }
    }

    pub fn or(sub_filters: Vec<SubFilter>) -> Self {
        Self {
            operator: ConditionOperator::OR,
            sub_filters,
        }
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

    pub fn order_by<T: ToString>(self, column: T, order: Order) -> Self {
        self.order_by_multiple(vec![column], order)
    }

    pub fn order_by_multiple<T: ToString>(mut self, columns: Vec<T>, order: Order) -> Self {
        self.order_by = Some(OrderBy {
            columns: columns.into_iter().map(|c| c.to_string()).collect(),
            order,
        });
        self
    }

    pub fn build(self) -> QueryFilter {
        QueryFilter {
            conditions: self.conditions,
            conditions_operator: self.conditions_operator.unwrap_or(ConditionOperator::AND),
            limit: self.limit,
            offset: self.offset,
            order_by: self.order_by,
        }
    }
}

impl From<SubFilter> for Condition {
    fn from(sub_filter: SubFilter) -> Self {
        Condition::filter(sub_filter)
    }
}

impl From<Vec<SubFilter>> for Condition {
    fn from(sub_filters: Vec<SubFilter>) -> Self {
        Condition::and(sub_filters)
    }
}

impl From<(Vec<SubFilter>, ConditionOperator)> for Condition {
    fn from((sub_filters, operator): (Vec<SubFilter>, ConditionOperator)) -> Self {
        match operator {
            ConditionOperator::AND => Condition::and(sub_filters),
            ConditionOperator::OR => Condition::or(sub_filters),
        }
    }
}

impl From<SubFilter> for QueryFilter {
    fn from(sub_filter: SubFilter) -> Self {
        QueryFilterBuilder::new().filter(sub_filter.into()).build()
    }
}

impl From<Vec<SubFilter>> for QueryFilter {
    fn from(sub_filters: Vec<SubFilter>) -> Self {
        QueryFilterBuilder::new().filter(sub_filters.into()).build()
    }
}

impl From<(Vec<SubFilter>, ConditionOperator)> for QueryFilter {
    fn from(sub_filters: (Vec<SubFilter>, ConditionOperator)) -> Self {
        QueryFilterBuilder::new().filter(sub_filters.into()).build()
    }
}

impl From<Condition> for QueryFilter {
    fn from(condition: Condition) -> Self {
        QueryFilterBuilder::new().filter(condition).build()
    }
}

impl From<Vec<Condition>> for QueryFilter {
    fn from(conditions: Vec<Condition>) -> Self {
        QueryFilterBuilder::new().filters(conditions).build()
    }
}

impl From<(Vec<Condition>, ConditionOperator)> for QueryFilter {
    fn from((conditions, operator): (Vec<Condition>, ConditionOperator)) -> Self {
        QueryFilterBuilder::new()
            .filters(conditions)
            .filter_operator(operator)
            .build()
    }
}
