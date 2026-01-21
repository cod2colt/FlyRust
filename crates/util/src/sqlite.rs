/// Score: SQLite
use chrono::Local;
use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

/// Score data model
///
/// Corresponds to the `score` table in SQLite
#[derive(Debug)]
struct Score {
    /// player name
    name: String,
    /// score
    score: i32,
    /// Creation timestamp (string format)
    created_at: String,
}

/// My SQLite structure
pub struct MyScore {
    pub conn: Connection,
    pub name: String,
    pub score: i32,
    pub output: String,
}
impl MyScore {
    /// Create new MySQLite
    pub fn new() -> rusqlite::Result<Self> {
        let db = Self::db_dir("flyrust.db");
        let conn = Connection::open(db)?;
        Self::init_db(&conn)?;

        Ok(Self {
            conn,
            name: "Guest".to_string(),
            score: 0,
            output: "Hello, Fly and Rust.".to_string(),
        })
    }

    /// Find the parent directory of the current executable
    /// and construct a full path to a database file located
    /// in the same directory as the application binary.
    ///
    /// # Arguments
    /// * `db_name` - SQLite database file name (e.g. `"flyplayer.db"`)
    ///
    /// # Returns
    /// A [`PathBuf`] pointing to:
    /// `<AppExecutableDir>/<db_name>`
    ///
    /// # Behavior
    /// - Resolves the absolute path of the running executable
    /// - Uses the executable's parent directory (not CWD)
    /// - Appends `db_name` to that directory
    ///
    /// # Panics
    /// - If the executable path cannot be determined
    /// - If the executable has no parent directory
    ///
    /// # Example
    /// ```no_run
    /// let db_path = db_dir("flyplayer.db");
    /// let conn = rusqlite::Connection::open(db_path)?;
    /// ```
    fn db_dir(db_name: &str) -> PathBuf {
        let exe = std::env::current_exe().expect("Failed to get executable path");

        // Windows: to open at the path is same with EXE
        // exe.parent()
        //     .expect("Failed to get executable directory")
        //     .join(db_name)

        // Mac: to open at the path is same with APP
        exe.parent() // MacOS
            .and_then(|p| p.parent()) // Contents
            .and_then(|p| p.parent()) // FlyRust.app
            .and_then(|p| p.parent()) // dist
            .expect("Failed to get executable directory")
            .join(db_name)
    }

    /// Initialize the SQLite database schema
    ///
    /// Creates the `score` table if it does not already exist.
    ///
    /// # Errors
    /// - Returns an error if the database write fails
    fn init_db(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS flyplayer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            score INTEGER NOT NULL,
            created_at TEXT NOT NULL
        )",
            [],
        )?;
        Ok(())
    }

    /// List all players
    pub fn list(&mut self) {
        let mut stmt = match self
            .conn
            .prepare("SELECT name, score, created_at FROM flyplayer ORDER BY score DESC;")
        {
            Ok(s) => s,
            Err(e) => return self.output.push_str(&e.to_string()),
        };

        let players = match stmt.query_map([], |row| {
            Ok(Score {
                name: row.get("name")?,
                score: row.get("score")?,
                created_at: row.get("created_at")?,
            })
        }) {
            Ok(s) => s,
            Err(e) => return self.output.push_str(&e.to_string()),
        };

        // get ranking list
        for (rank, player) in players.enumerate() {
            let t = match player {
                Ok(n) => n,
                Err(e) => {
                    return self.output.push_str(&format!("{}", e));
                }
            };

            let ranking = rank + 1;

            self.output.push_str(&format!(
                "\n          {:^4}  {:>3}    {:^10}   {:^19}",
                ranking, t.score, t.name, t.created_at
            ));
        }
    }

    /// Add a new player score
    pub fn add(&mut self) {
        // check input valid
        let title = self.name.trim();
        if title.is_empty() {
            return;
        }

        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // INSERT
        if let Err(e) = self.conn.execute(
            "INSERT INTO flyplayer (name, score, created_at) VALUES (?1, ?2, ?3)",
            params![title, self.score, now],
        ) {
            return self.output.push_str(&format!("❗ DB error: {}\n", e));
        }

        // keep 10 players
        let _ = self.conn.execute(
            r#"
            DELETE FROM flyplayer
            WHERE id NOT IN (
            SELECT id
            FROM flyplayer
            ORDER BY score DESC
            LIMIT 10
        )
        "#,
            [],
        );
    }
}
