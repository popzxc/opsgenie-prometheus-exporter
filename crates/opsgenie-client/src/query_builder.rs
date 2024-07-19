#[derive(Debug)]
pub struct Query {
    field: String,
    operator: Operator,
    filter: Box<dyn ToFilter>,
}

pub trait ToFilter: 'static + std::fmt::Debug {
    fn to_filter(&self) -> String;

    fn and<R: ToFilter>(self, other: R) -> And
    where
        Self: Sized,
    {
        And::new(self, other)
    }

    fn or<R: ToFilter>(self, other: R) -> Or
    where
        Self: Sized,
    {
        Or::new(self, other)
    }

    fn not(self) -> Not
    where
        Self: Sized,
    {
        Not {
            value: self.to_filter(),
        }
    }

    fn wildcard(self) -> Wildcard
    where
        Self: Sized,
    {
        Wildcard {
            value: self.to_filter(),
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    ExactMatch,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

impl Operator {
    fn as_symbol(&self) -> &str {
        match self {
            Operator::ExactMatch => ":",
            Operator::Less => "<",
            Operator::LessOrEqual => "<=",
            Operator::Greater => ">",
            Operator::GreaterOrEqual => ">=",
        }
    }
}

impl Query {
    fn with_operator(field: String, filter: Box<dyn ToFilter>, operator: Operator) -> Self {
        Self {
            field,
            operator,
            filter,
        }
    }

    pub fn new<T: ToString, F: ToFilter>(field: T, filter: F) -> Self {
        Self {
            field: field.to_string(),
            operator: Operator::ExactMatch,
            filter: Box::new(filter),
        }
    }

    pub fn less<T: ToString, F: ToFilter>(field: T, filter: F) -> Self {
        Self::with_operator(field.to_string(), Box::new(filter), Operator::Less)
    }

    pub fn less_or_equal<T: ToString, F: ToFilter>(field: T, filter: F) -> Self {
        Self::with_operator(field.to_string(), Box::new(filter), Operator::LessOrEqual)
    }

    pub fn greater<T: ToString, F: ToFilter>(field: T, filter: F) -> Self {
        Self::with_operator(field.to_string(), Box::new(filter), Operator::Greater)
    }

    pub fn greater_or_equal<T: ToString, F: ToFilter>(field: T, filter: F) -> Self {
        Self::with_operator(
            field.to_string(),
            Box::new(filter),
            Operator::GreaterOrEqual,
        )
    }
}

impl ToFilter for Query {
    fn to_filter(&self) -> String {
        format!(
            "{}{}{}",
            self.field,
            self.operator.as_symbol(),
            self.filter.to_filter()
        )
    }
}

impl<T: std::fmt::Debug + ToString + 'static> ToFilter for T {
    fn to_filter(&self) -> String {
        let str = self.to_string();
        if str.contains(' ') {
            format!("\"{}\"", str)
        } else {
            str
        }
    }
}

#[derive(Debug)]
pub struct Wildcard {
    value: String,
}

impl ToFilter for Wildcard {
    fn to_filter(&self) -> String {
        format!("{}*", self.value)
    }
}

#[derive(Debug)]
pub struct Not {
    value: String,
}

impl ToFilter for Not {
    fn to_filter(&self) -> String {
        format!("NOT {}", self.value)
    }
}

#[derive(Debug)]
pub struct And {
    left: Box<dyn ToFilter>,
    right: Box<dyn ToFilter>,
}

impl And {
    fn new<L: ToFilter, R: ToFilter>(left: L, right: R) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl ToFilter for And {
    fn to_filter(&self) -> String {
        format!(
            "({}) AND ({})",
            self.left.to_filter(),
            self.right.to_filter()
        )
    }
}

#[derive(Debug)]
pub struct Or {
    left: Box<dyn ToFilter>,
    right: Box<dyn ToFilter>,
}

impl Or {
    fn new<L: ToFilter, R: ToFilter>(left: L, right: R) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl ToFilter for Or {
    fn to_filter(&self) -> String {
        format!(
            "({}) OR ({})",
            self.left.to_filter(),
            self.right.to_filter()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_query() {
        let query = Query::new("field", "value");
        assert_eq!(query.to_filter(), "(field:value)");
        let query = Query::less("field", 42);
        assert_eq!(query.to_filter(), "(field<42)");
        let query = Query::greater("field", 42);
        assert_eq!(query.to_filter(), "(field>42)");
        let query = Query::greater_or_equal("field", 42);
        assert_eq!(query.to_filter(), "(field>=42)");
        let query = Query::less_or_equal("field", 42);
        assert_eq!(query.to_filter(), "(field<=42)");
    }

    #[test]
    fn combinations() {
        let query = Query::new("field", "value")
            .and(Query::less("field2", 42).not())
            .or(Query::new("field3", "lorem".or("ipsum".wildcard())));
        assert_eq!(
            query.to_filter(),
            "((field:value) AND (NOT field2<42)) OR (field3:(lorem) OR (ipsum*))"
        );
    }
}
