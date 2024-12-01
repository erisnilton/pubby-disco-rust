#[macro_export]
macro_rules! filter_params {
    ($qb: expr, $query:expr, {
      $($name:ident($value:ident) => $filter:expr,)*
    }) => {
      let mut has_filters = false;

      $(
        if let Some(value) = &$query.$name {
          if has_filters {
            $qb.push(" AND ");
          }

          $qb.push("(");
          {
            let $value = value;
            $filter;
          }
          $qb.push(")");
          #[warn(unused_assignments)]
          {has_filters = true}
        }
      )*
    };
}

#[macro_export]
macro_rules! where_filter {
  ($qb: expr, $query:expr, {
    $($name:ident($value:ident) => $filter:expr,)*
  }) => {
      if $($query.$name.is_some() || )* false {
        $qb.push(" WHERE ");
      }

      filter_params! {
        $qb,
        $query,
        {
          $($name($value) => $filter,)*
        }
      }
  };
}

#[macro_export]
macro_rules! count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + count!($($tail)*) };
}

#[macro_export]
macro_rules! search_by {
  ($qb:expr, $value:expr, $($field:literal$(,)?)*$(,)?) => {
    let mut count = count!($($field)*);

    $(
      $qb.push($field);
      $qb.push(" ILIKE '%' || ");
      $qb.push_bind($value);
      $qb.push(" || '%'");

      if count > 1 {
        $qb.push(" OR ");
      }
      #[warn(unused_assignments)]
      {count -= 1;}
    )*
  };
}

#[macro_export]
macro_rules! create_filter {
    ($query:ty, $qb: ident => {
      $($name:ident($value:ident) => $filter:expr,)*
    }) => {
        |$qb: &mut sqlx::QueryBuilder<'_, sqlx::Postgres>, query: &$query| {
            where_filter! {
              $qb,
              query,
              {
                $($name($value) => $filter,)*
              }
            }
        }
    };
}
