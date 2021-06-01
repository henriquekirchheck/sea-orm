use crate::{ColumnTrait, Identity, IntoSimpleExpr, RelationDef};
use sea_query::{Alias, Expr, IntoCondition, SelectExpr, SelectStatement, SimpleExpr};
pub use sea_query::{Condition, JoinType, Order};
use std::rc::Rc;

pub trait SelectHelper: Sized {
    fn query(&mut self) -> &mut SelectStatement;

    /// Clear the selection list
    fn select_only(mut self) -> Self {
        self.query().clear_selects();
        self
    }

    /// Add a select column
    /// ```
    /// use sea_orm::{ColumnTrait, EntityTrait, QueryTrait, SelectHelper, tests_cfg::cake, sea_query::PostgresQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column(cake::Column::Name)
    ///         .build(PostgresQueryBuilder)
    ///         .to_string(),
    ///     r#"SELECT "cake"."name" FROM "cake""#
    /// );
    /// ```
    fn column<C>(mut self, col: C) -> Self
    where
        C: ColumnTrait,
    {
        self.query().expr(col.into_simple_expr());
        self
    }

    /// Add a select column with alias
    /// ```
    /// use sea_orm::{ColumnTrait, EntityTrait, QueryTrait, SelectHelper, tests_cfg::cake, sea_query::PostgresQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column_as(cake::Column::Id.count(), "count")
    ///         .build(PostgresQueryBuilder)
    ///         .to_string(),
    ///     r#"SELECT COUNT("cake"."id") AS "count" FROM "cake""#
    /// );
    /// ```
    fn column_as<C>(mut self, col: C, alias: &str) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query().expr(SelectExpr {
            expr: col.into_simple_expr(),
            alias: Some(Rc::new(Alias::new(alias))),
        });
        self
    }

    /// Add an AND WHERE expression
    /// ```
    /// use sea_orm::{ColumnTrait, EntityTrait, QueryTrait, SelectHelper, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Id.eq(4))
    ///         .filter(cake::Column::Id.eq(5))
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` = 4 AND `cake`.`id` = 5"
    /// );
    /// ```
    ///
    /// Add a condition tree.
    /// ```
    /// use sea_orm::{Condition, ColumnTrait, EntityTrait, QueryTrait, SelectHelper, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(
    ///             Condition::any()
    ///                 .add(cake::Column::Id.eq(4))
    ///                 .add(cake::Column::Id.eq(5))
    ///         )
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` = 4 OR `cake`.`id` = 5"
    /// );
    /// ```
    fn filter<F>(mut self, filter: F) -> Self
    where
        F: IntoCondition,
    {
        self.query().cond_where(filter.into_condition());
        self
    }

    /// Add a group by column
    /// ```
    /// use sea_orm::{ColumnTrait, EntityTrait, QueryTrait, SelectHelper, tests_cfg::cake, sea_query::PostgresQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .select_only()
    ///         .column(cake::Column::Name)
    ///         .group_by(cake::Column::Name)
    ///         .build(PostgresQueryBuilder)
    ///         .to_string(),
    ///     r#"SELECT "cake"."name" FROM "cake" GROUP BY "cake"."name""#
    /// );
    /// ```
    fn group_by<C>(mut self, col: C) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query().add_group_by(vec![col.into_simple_expr()]);
        self
    }

    /// Add an order_by expression
    /// ```
    /// use sea_orm::{EntityTrait, Order, QueryTrait, SelectHelper, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .order_by(cake::Column::Id, Order::Asc)
    ///         .order_by(cake::Column::Name, Order::Desc)
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` ORDER BY `cake`.`id` ASC, `cake`.`name` DESC"
    /// );
    /// ```
    fn order_by<C>(mut self, col: C, ord: Order) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query().order_by_expr(col.into_simple_expr(), ord);
        self
    }

    /// Add an order_by expression (ascending)
    /// ```
    /// use sea_orm::{EntityTrait, QueryTrait, SelectHelper, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .order_by_asc(cake::Column::Id)
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` ORDER BY `cake`.`id` ASC"
    /// );
    /// ```
    fn order_by_asc<C>(mut self, col: C) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query()
            .order_by_expr(col.into_simple_expr(), Order::Asc);
        self
    }

    /// Add an order_by expression (descending)
    /// ```
    /// use sea_orm::{EntityTrait, QueryTrait, SelectHelper, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .order_by_desc(cake::Column::Id)
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` ORDER BY `cake`.`id` DESC"
    /// );
    /// ```
    fn order_by_desc<C>(mut self, col: C) -> Self
    where
        C: IntoSimpleExpr,
    {
        self.query()
            .order_by_expr(col.into_simple_expr(), Order::Desc);
        self
    }

    #[doc(hidden)]
    fn join_join(mut self, join: JoinType, rel: RelationDef, via: Option<RelationDef>) -> Self {
        if let Some(via) = via {
            self = self.join(join, via)
        }
        self.join(join, rel)
    }

    #[doc(hidden)]
    fn join_join_rev(mut self, join: JoinType, rel: RelationDef, via: Option<RelationDef>) -> Self {
        self = self.join_rev(join, rel);
        if let Some(via) = via {
            self = self.join_rev(join, via)
        }
        self
    }

    /// Join via [`RelationDef`].
    fn join(mut self, join: JoinType, rel: RelationDef) -> Self {
        self.query()
            .join(join, rel.to_tbl.clone(), join_condition(rel));
        self
    }

    /// Join via [`RelationDef`] but in reverse direction.
    /// Assume when there exist a relation A to B.
    /// You can reverse join B from A.
    fn join_rev(mut self, join: JoinType, rel: RelationDef) -> Self {
        self.query()
            .join(join, rel.from_tbl.clone(), join_condition(rel));
        self
    }
}

fn join_condition(rel: RelationDef) -> SimpleExpr {
    let from_tbl = rel.from_tbl.clone();
    let to_tbl = rel.to_tbl.clone();
    let owner_keys = rel.from_col;
    let foreign_keys = rel.to_col;

    match (owner_keys, foreign_keys) {
        (Identity::Unary(o1), Identity::Unary(f1)) => {
            Expr::tbl(Rc::clone(&from_tbl), o1).equals(Rc::clone(&to_tbl), f1)
        }
        (Identity::Binary(o1, o2), Identity::Binary(f1, f2)) => Expr::tbl(Rc::clone(&from_tbl), o1)
            .equals(Rc::clone(&to_tbl), f1)
            .and(Expr::tbl(Rc::clone(&from_tbl), o2).equals(Rc::clone(&to_tbl), f2)),
        _ => panic!("Owner key and foreign key mismatch"),
    }
}
