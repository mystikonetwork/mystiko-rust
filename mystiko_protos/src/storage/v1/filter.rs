use crate::storage::v1::{ColumnValue, Condition, ConditionOperator, QueryFilter, SubFilter, SubFilterOperator};

impl SubFilter {
    pub fn is_null<C: ToString>(column: C) -> Self {
        Self {
            operator: SubFilterOperator::IsNull as i32,
            column: column.to_string(),
            values: vec![],
        }
    }

    pub fn is_not_null<C: ToString>(column: C) -> Self {
        Self {
            operator: SubFilterOperator::IsNotNull as i32,
            column: column.to_string(),
            values: vec![],
        }
    }

    pub fn equal<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::Equal as i32,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }

    pub fn not_equal<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::NotEqual as i32,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }

    pub fn greater<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::Greater as i32,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }

    pub fn greater_equal<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::GreaterEqual as i32,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }

    pub fn less<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::Less as i32,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }

    pub fn less_equal<C: ToString, V: Into<ColumnValue>>(column: C, value: V) -> Self {
        Self {
            operator: SubFilterOperator::LessEqual as i32,
            column: column.to_string(),
            values: vec![value.into()],
        }
    }

    pub fn between_and<C: ToString, V: Into<ColumnValue>>(column: C, value1: V, value2: V) -> Self {
        Self {
            operator: SubFilterOperator::BetweenAnd as i32,
            column: column.to_string(),
            values: vec![value1.into(), value2.into()],
        }
    }

    pub fn in_list<C: ToString, V: Into<ColumnValue>>(column: C, values: Vec<V>) -> Self {
        Self {
            operator: SubFilterOperator::In as i32,
            column: column.to_string(),
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl Condition {
    pub fn filter(sub_filter: SubFilter) -> Self {
        Self {
            operator: ConditionOperator::And as i32,
            sub_filters: vec![sub_filter],
        }
    }

    pub fn and(sub_filters: Vec<SubFilter>) -> Self {
        Self {
            operator: ConditionOperator::And as i32,
            sub_filters,
        }
    }

    pub fn or(sub_filters: Vec<SubFilter>) -> Self {
        Self {
            operator: ConditionOperator::Or as i32,
            sub_filters,
        }
    }
}

impl QueryFilter {
    pub fn has_conditions(&self) -> bool {
        !self.conditions.is_empty() || self.additional_condition.is_some()
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
            ConditionOperator::And => Condition::and(sub_filters),
            ConditionOperator::Or => Condition::or(sub_filters),
            _ => Condition::default(),
        }
    }
}

impl From<SubFilter> for QueryFilter {
    fn from(sub_filter: SubFilter) -> Self {
        QueryFilter::builder()
            .conditions(vec![sub_filter.into()])
            .conditions_operator(ConditionOperator::And as i32)
            .build()
    }
}

impl From<Vec<SubFilter>> for QueryFilter {
    fn from(sub_filters: Vec<SubFilter>) -> Self {
        QueryFilter::builder()
            .conditions(vec![sub_filters.into()])
            .conditions_operator(ConditionOperator::And as i32)
            .build()
    }
}

impl From<(Vec<SubFilter>, ConditionOperator)> for QueryFilter {
    fn from(sub_filters: (Vec<SubFilter>, ConditionOperator)) -> Self {
        QueryFilter::builder()
            .conditions(vec![sub_filters.into()])
            .conditions_operator(ConditionOperator::And as i32)
            .build()
    }
}

impl From<Condition> for QueryFilter {
    fn from(condition: Condition) -> Self {
        QueryFilter::builder()
            .conditions(vec![condition])
            .conditions_operator(ConditionOperator::And as i32)
            .build()
    }
}

impl From<Vec<Condition>> for QueryFilter {
    fn from(conditions: Vec<Condition>) -> Self {
        QueryFilter::builder()
            .conditions(conditions)
            .conditions_operator(ConditionOperator::And as i32)
            .build()
    }
}

impl From<(Vec<Condition>, ConditionOperator)> for QueryFilter {
    fn from((conditions, operator): (Vec<Condition>, ConditionOperator)) -> Self {
        QueryFilter::builder()
            .conditions(conditions)
            .conditions_operator(operator)
            .build()
    }
}
