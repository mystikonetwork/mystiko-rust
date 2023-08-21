use mystiko_protos::storage::v1::column_value::Value;
use mystiko_protos::storage::v1::{
    ColumnValue, Condition, ConditionOperator, Order, OrderBy, QueryFilter, SubFilter, SubFilterOperator,
};

#[test]
fn test_equal_filter() {
    let equal_filter = SubFilter::equal("column1", 123i32);
    assert_eq!(equal_filter.operator, SubFilterOperator::Equal as i32);
    assert_eq!(equal_filter.column, "column1");
    assert_eq!(
        equal_filter.values,
        vec![ColumnValue::builder().value(Value::I32Value(123)).build()]
    );
}

#[test]
fn test_not_equal_filter() {
    let not_equal_filter = SubFilter::not_equal("column1", 123i32);
    assert_eq!(not_equal_filter.operator, SubFilterOperator::NotEqual as i32);
    assert_eq!(not_equal_filter.column, "column1");
    assert_eq!(
        not_equal_filter.values,
        vec![ColumnValue::builder().value(Value::I32Value(123)).build()]
    );
}

#[test]
fn test_less_filter() {
    let less_filter = SubFilter::less("column1", 123i32);
    assert_eq!(less_filter.operator, SubFilterOperator::Less as i32);
    assert_eq!(less_filter.column, "column1");
    assert_eq!(
        less_filter.values,
        vec![ColumnValue::builder().value(Value::I32Value(123)).build()]
    );
}

#[test]
fn test_less_equal_filter() {
    let less_equal_filter = SubFilter::less_equal("column1", 123i32);
    assert_eq!(less_equal_filter.operator, SubFilterOperator::LessEqual as i32);
    assert_eq!(less_equal_filter.column, "column1");
    assert_eq!(
        less_equal_filter.values,
        vec![ColumnValue::builder().value(Value::I32Value(123)).build()]
    );
}

#[test]
fn test_greater_filter() {
    let greater_filter = SubFilter::greater("column1", 123i32);
    assert_eq!(greater_filter.operator, SubFilterOperator::Greater as i32);
    assert_eq!(greater_filter.column, "column1");
    assert_eq!(
        greater_filter.values,
        vec![ColumnValue::builder().value(Value::I32Value(123)).build()]
    );
}

#[test]
fn test_greater_equal_filter() {
    let greater_equal_filter = SubFilter::greater_equal("column1", 123i32);
    assert_eq!(greater_equal_filter.operator, SubFilterOperator::GreaterEqual as i32);
    assert_eq!(greater_equal_filter.column, "column1");
    assert_eq!(
        greater_equal_filter.values,
        vec![ColumnValue::builder().value(Value::I32Value(123)).build()]
    );
}

#[test]
fn test_between_and_filter() {
    let between_and_filter = SubFilter::between_and("column1", 123i32, 456i32);
    assert_eq!(between_and_filter.operator, SubFilterOperator::BetweenAnd as i32);
    assert_eq!(between_and_filter.column, "column1");
    assert_eq!(
        between_and_filter.values,
        vec![
            ColumnValue::builder().value(Value::I32Value(123)).build(),
            ColumnValue::builder().value(Value::I32Value(456)).build()
        ]
    );
}

#[test]
fn test_in_list_filter() {
    let in_list_filter = SubFilter::in_list("column1", vec![123i32, 456i32]);
    assert_eq!(in_list_filter.operator, SubFilterOperator::In as i32);
    assert_eq!(in_list_filter.column, "column1");
    assert_eq!(
        in_list_filter.values,
        vec![
            ColumnValue::builder().value(Value::I32Value(123)).build(),
            ColumnValue::builder().value(Value::I32Value(456)).build()
        ]
    );
}

#[test]
fn test_is_null_filter() {
    let is_null_filter = SubFilter::is_null("column1");
    assert_eq!(is_null_filter.operator, SubFilterOperator::IsNull as i32);
    assert_eq!(is_null_filter.column, "column1");
    assert_eq!(is_null_filter.values, vec![]);
}

#[test]
fn test_is_not_null_filter() {
    let is_not_null_filter = SubFilter::is_not_null("column1");
    assert_eq!(is_not_null_filter.operator, SubFilterOperator::IsNotNull as i32);
    assert_eq!(is_not_null_filter.column, "column1");
    assert_eq!(is_not_null_filter.values, vec![]);
}

#[test]
fn test_and_condition() {
    let sub_filters = vec![SubFilter::equal("column1", 123i32), SubFilter::equal("column2", 456i32)];
    let condition = Condition::and(sub_filters.clone());
    assert_eq!(condition.operator, ConditionOperator::And as i32);
    assert_eq!(condition.sub_filters, sub_filters);
}

#[test]
fn test_or_condition() {
    let sub_filters = vec![SubFilter::equal("column1", 123i32), SubFilter::equal("column2", 456i32)];
    let condition = Condition::or(sub_filters.clone());
    assert_eq!(condition.operator, ConditionOperator::Or as i32);
    assert_eq!(condition.sub_filters, sub_filters);
}

#[test]
fn test_single_filter_condition() {
    let sub_filter = SubFilter::equal("column1", 123i32);
    let condition = Condition::filter(sub_filter.clone());
    assert_eq!(condition.operator, ConditionOperator::And as i32);
    assert_eq!(condition.sub_filters, vec![sub_filter]);
}

#[test]
fn test_query_filter_builder() {
    let conditions: Vec<Condition> = vec![
        Condition::filter(SubFilter::equal("column1", 123i32)),
        Condition::filter(SubFilter::equal("column2", 456i32)),
    ];
    let query_filter = QueryFilter::builder()
        .conditions(conditions.clone())
        .conditions_operator(ConditionOperator::Or)
        .limit(10)
        .offset(20)
        .order_by(
            OrderBy::builder()
                .columns(vec!["column1".to_string()])
                .order(Order::Desc)
                .build(),
        )
        .build();
    assert_eq!(query_filter.conditions, conditions);
    assert_eq!(query_filter.conditions_operator, ConditionOperator::Or as i32);
    assert_eq!(query_filter.limit.unwrap(), 10);
    assert_eq!(query_filter.offset.unwrap(), 20);
    assert_eq!(query_filter.order_by.as_ref().unwrap().order, Order::Desc as i32);
    assert_eq!(query_filter.order_by.as_ref().unwrap().columns, vec!["column1"]);
}

#[test]
fn test_sub_filter_to_condition() {
    let sub_filters = vec![SubFilter::equal("column1", 123i32), SubFilter::equal("column2", 345i32)];
    let condition: Condition = sub_filters[0].clone().into();
    assert_eq!(condition.operator, ConditionOperator::And as i32);
    assert_eq!(condition.sub_filters, sub_filters[..1]);
    let condition: Condition = sub_filters.clone().into();
    assert_eq!(condition.operator, ConditionOperator::And as i32);
    assert_eq!(condition.sub_filters, sub_filters);
    let condition: Condition = (sub_filters.clone(), ConditionOperator::Or).into();
    assert_eq!(condition.operator, ConditionOperator::Or as i32);
    assert_eq!(condition.sub_filters, sub_filters);
}

#[test]
fn test_sub_filter_to_query_filter() {
    let sub_filters = vec![SubFilter::equal("column1", 123i32), SubFilter::equal("column2", 345i32)];
    let condition: Condition = sub_filters[0].clone().into();
    let query_filter: QueryFilter = sub_filters[0].clone().into();
    assert_eq!(query_filter.conditions, vec![condition]);
    assert_eq!(query_filter.conditions_operator, ConditionOperator::And as i32);
    assert!(query_filter.limit.is_none());
    assert!(query_filter.offset.is_none());
    assert!(query_filter.order_by.is_none());
    let condition: Condition = sub_filters.clone().into();
    let query_filter: QueryFilter = sub_filters.clone().into();
    assert_eq!(query_filter.conditions, vec![condition]);
    assert_eq!(query_filter.conditions_operator, ConditionOperator::And as i32);
    assert!(query_filter.limit.is_none());
    assert!(query_filter.offset.is_none());
    assert!(query_filter.order_by.is_none());
    let condition: Condition = (sub_filters.clone(), ConditionOperator::Or).into();
    let query_filter: QueryFilter = (sub_filters, ConditionOperator::Or).into();
    assert_eq!(query_filter.conditions, vec![condition]);
    assert_eq!(query_filter.conditions_operator, ConditionOperator::And as i32);
    assert!(query_filter.limit.is_none());
    assert!(query_filter.offset.is_none());
    assert!(query_filter.order_by.is_none());
}

#[test]
fn test_condition_to_query_filter() {
    let conditions: Vec<Condition> = vec![
        SubFilter::equal("column1", 123i32).into(),
        vec![SubFilter::equal("column2", 345i32), SubFilter::equal("column3", 567i32)].into(),
    ];
    let query_filter: QueryFilter = conditions[0].clone().into();
    assert_eq!(query_filter.conditions, conditions[..1]);
    assert_eq!(query_filter.conditions_operator, ConditionOperator::And as i32);
    assert!(query_filter.limit.is_none());
    assert!(query_filter.offset.is_none());
    assert!(query_filter.order_by.is_none());
    let query_filter: QueryFilter = conditions.clone().into();
    assert_eq!(query_filter.conditions, conditions);
    assert_eq!(query_filter.conditions_operator, ConditionOperator::And as i32);
    assert!(query_filter.limit.is_none());
    assert!(query_filter.offset.is_none());
    assert!(query_filter.order_by.is_none());
    let query_filter: QueryFilter = (conditions.clone(), ConditionOperator::Or).into();
    assert_eq!(query_filter.conditions, conditions);
    assert_eq!(query_filter.conditions_operator, ConditionOperator::Or as i32);
    assert!(query_filter.limit.is_none());
    assert!(query_filter.offset.is_none());
    assert!(query_filter.order_by.is_none());
}
