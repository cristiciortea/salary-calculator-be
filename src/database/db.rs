use super::db_backup::get_initial_insert_statements;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct TaxRates {
    pub year: i32,
    pub income_tax: f64,
    pub social_security: f64,
    pub health_insurance: f64,
    pub insurance_contribution: f64,
}

// Function to create the table and insert some data if it does not exist.
pub fn setup_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tax_rates (
            year INTEGER PRIMARY KEY,
            income_tax REAL NOT NULL,
            social_security REAL NOT NULL,
            health_insurance REAL NOT NULL,
            insurance_contribution REAL NOT NULL
        )",
        [],
    )?;

    let tax_rates_row_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM tax_rates", [], |row| row.get(0))?;

    if tax_rates_row_count == 0 {
        // Table is empty, insert initial data.
        let statements = get_initial_insert_statements();
        for statement in statements {
            println!("[DEBUG]: Executing statement: {}", statement);
            conn.execute(statement.as_str(), [])?;
        }
    }

    Ok(())
}

// Function to query the tax rates for a specific year.
pub fn get_tax_rates(conn: &Connection, year: i32) -> Option<TaxRates> {
    let mut stmt = conn
        .prepare(
            "SELECT year, income_tax, social_security, health_insurance, insurance_contribution
         FROM tax_rates WHERE year = ?1",
        )
        .expect("Connection should work.");

    let tax_rates = match stmt.query_row(params![year], |row| {
        Ok(TaxRates {
            year: row.get(0)?,
            income_tax: row.get(1)?,
            social_security: row.get(2)?,
            health_insurance: row.get(3)?,
            insurance_contribution: row.get(4)?,
        })
    }) {
        Ok(tax_rate) => Some(tax_rate),
        Err(_) => None,
    };

    tax_rates
}
