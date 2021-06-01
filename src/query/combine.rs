use crate::{EntityTrait, IntoSimpleExpr, Iterable, QueryTrait, Select, SelectTwo};
use core::marker::PhantomData;
pub use sea_query::JoinType;
use sea_query::{Alias, ColumnRef, Iden, SelectExpr, SelectStatement, SimpleExpr};
use std::rc::Rc;

pub const SELECT_A: &str = "A_";
pub const SELECT_B: &str = "B_";

impl<E> Select<E>
where
    E: EntityTrait,
{
    fn apply_alias(mut self, pre: &str) -> Self {
        self.query().exprs_mut_for_each(|sel| {
            match &sel.alias {
                Some(alias) => {
                    let alias = format!("{}{}", pre, alias.to_string().as_str());
                    sel.alias = Some(Rc::new(Alias::new(&alias)));
                }
                None => {
                    let col = match &sel.expr {
                        SimpleExpr::Column(col_ref) => match &col_ref {
                            ColumnRef::Column(col) => col,
                            ColumnRef::TableColumn(_, col) => col,
                        },
                        _ => panic!("cannot apply alias for expr other than Column"),
                    };
                    let alias = format!("{}{}", pre, col.to_string().as_str());
                    sel.alias = Some(Rc::new(Alias::new(&alias)));
                }
            };
        });
        self
    }

    pub fn select_also<F>(mut self, _: F) -> SelectTwo<E, F>
    where
        F: EntityTrait,
    {
        self = self.apply_alias(SELECT_A);
        SelectTwo::new(self.into_query())
    }
}

impl<E, F> SelectTwo<E, F>
where
    E: EntityTrait,
    F: EntityTrait,
{
    pub(crate) fn new(query: SelectStatement) -> Self {
        let myself = Self {
            query,
            entity: PhantomData,
        };
        myself.prepare_select()
    }

    fn prepare_select(mut self) -> Self {
        for col in <F::Column as Iterable>::iter() {
            let alias = format!("{}{}", SELECT_B, col.to_string().as_str());
            self.query.expr(SelectExpr {
                expr: col.into_simple_expr(),
                alias: Some(Rc::new(Alias::new(&alias))),
            });
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_cfg::{cake, fruit};
    use crate::{ColumnTrait, EntityTrait, QueryTrait, SelectHelper};
    use sea_query::MysqlQueryBuilder;

    #[test]
    fn alias_1() {
        assert_eq!(
            cake::Entity::find()
                .column_as(cake::Column::Id, "B")
                .apply_alias("A_")
                .build(MysqlQueryBuilder)
                .to_string(),
            "SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`, `cake`.`id` AS `A_B` FROM `cake`",
        );
    }

    #[test]
    fn select_also_1() {
        assert_eq!(
            cake::Entity::find()
                .left_join(fruit::Entity)
                .select_also(fruit::Entity)
                .build(MysqlQueryBuilder)
                .to_string(),
            [
                "SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,",
                "`fruit`.`id` AS `B_id`, `fruit`.`name` AS `B_name`, `fruit`.`cake_id` AS `B_cake_id`",
                "FROM `cake` LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id`",
            ].join(" ")
        );
    }

    #[test]
    fn select_also_2() {
        assert_eq!(
            cake::Entity::find()
                .left_join(fruit::Entity)
                .select_also(fruit::Entity)
                .filter(cake::Column::Id.eq(1))
                .filter(fruit::Column::Id.eq(2))
                .build(MysqlQueryBuilder)
                .to_string(),
            [
                "SELECT `cake`.`id` AS `A_id`, `cake`.`name` AS `A_name`,",
                "`fruit`.`id` AS `B_id`, `fruit`.`name` AS `B_name`, `fruit`.`cake_id` AS `B_cake_id`",
                "FROM `cake` LEFT JOIN `fruit` ON `cake`.`id` = `fruit`.`cake_id`",
                "WHERE `cake`.`id` = 1 AND `fruit`.`id` = 2",
            ].join(" ")
        );
    }
}
