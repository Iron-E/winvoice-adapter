use sqlx::{Executor, Result};
use winvoice_match::MatchExpense;
use winvoice_schema::{
	chrono::{DateTime, Utc},
	Expense,
	Id,
	Money,
};

use crate::{Deletable, Retrievable, Updatable};

/// Implementors of this trait may act as an [adapter](super) for [`Employee`]s.
#[async_trait::async_trait]
pub trait ExpensesAdapter:
	Deletable<Entity = Expense>
	+ Retrievable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity, Match = MatchExpense>
	+ Updatable<Db = <Self as Deletable>::Db, Entity = <Self as Deletable>::Entity>
{
	/// Initialize and return new [`Expense`]s via the `connection`.
	///
	/// # Parameters
	///
	/// * `expenses` is a list of `(String, Money, String)`, which represents `(category, cost, description)` for the
	///   created [`Expense`]s.
	async fn create<'connection, Conn>(
		connection: Conn,
		expenses: Vec<(String, Money, String)>,
		timesheet: (Id, DateTime<Utc>),
	) -> Result<Vec<Expense>>
	where
		Conn: Executor<'connection, Database = <Self as Deletable>::Db>;
}
