use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::Mutex;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("queue storage: {0}")]
    Storage(#[from] rusqlite::Error),
    #[error("queue lock poisoned")]
    Poisoned,
    #[error("unknown job or invalid state transition")]
    InvalidTransition,
    #[error("invalid application ID")]
    InvalidAppId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Compatforge,
    Flatpak,
    ForgePackage,
    UbuntuDeb,
    Snap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Install,
    Update,
    Uninstall,
    Rollback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Queued,
    Running,
    Cancelling,
    Interrupted,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub id: String,
    pub app_id: String,
    pub backend: Backend,
    pub action: Action,
    pub state: JobState,
    pub detail: Option<String>,
    pub native_pending: bool,
}

pub struct JobQueue {
    conn: Mutex<Connection>,
}

impl Backend {
    fn as_str(self) -> &'static str {
        match self {
            Self::Compatforge => "compatforge",
            Self::Flatpak => "flatpak",
            Self::ForgePackage => "forge-package",
            Self::UbuntuDeb => "ubuntu-deb",
            Self::Snap => "snap",
        }
    }
    fn parse(value: &str) -> rusqlite::Result<Self> {
        match value {
            "compatforge" => Ok(Self::Compatforge),
            "flatpak" => Ok(Self::Flatpak),
            "forge-package" => Ok(Self::ForgePackage),
            "ubuntu-deb" => Ok(Self::UbuntuDeb),
            "snap" => Ok(Self::Snap),
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }
}

impl Action {
    fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Update => "update",
            Self::Uninstall => "uninstall",
            Self::Rollback => "rollback",
        }
    }
    fn parse(value: &str) -> rusqlite::Result<Self> {
        match value {
            "install" => Ok(Self::Install),
            "update" => Ok(Self::Update),
            "uninstall" => Ok(Self::Uninstall),
            "rollback" => Ok(Self::Rollback),
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }
}

impl JobState {
    fn parse(value: &str) -> rusqlite::Result<Self> {
        match value {
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "cancelling" => Ok(Self::Cancelling),
            "interrupted" => Ok(Self::Interrupted),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(rusqlite::Error::InvalidQuery),
        }
    }
}

fn row_job(row: &rusqlite::Row<'_>) -> rusqlite::Result<Job> {
    let backend: String = row.get(2)?;
    let action: String = row.get(3)?;
    let state: String = row.get(4)?;
    Ok(Job {
        id: row.get(0)?,
        app_id: row.get(1)?,
        backend: Backend::parse(&backend)?,
        action: Action::parse(&action)?,
        state: JobState::parse(&state)?,
        detail: row.get(5)?,
        native_pending: row.get(6)?,
    })
}

const COLUMNS: &str = "id,app_id,backend,action,state,detail,native_pending";

impl JobQueue {
    pub fn open(path: &Path) -> Result<Self, QueueError> {
        let conn = Connection::open(path)?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;\
             CREATE TABLE IF NOT EXISTS jobs (sequence INTEGER PRIMARY KEY AUTOINCREMENT, id TEXT NOT NULL UNIQUE,\
             app_id TEXT NOT NULL, backend TEXT NOT NULL, action TEXT NOT NULL, state TEXT NOT NULL, detail TEXT);\
             ")?;
        let has_pending = {
            let mut stmt = conn.prepare("PRAGMA table_info(jobs)")?;
            let names = stmt
                .query_map([], |row| row.get::<_, String>(1))?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            names.iter().any(|name| name == "native_pending")
        };
        if !has_pending {
            conn.execute_batch("ALTER TABLE jobs ADD COLUMN native_pending INTEGER NOT NULL DEFAULT 0 CHECK(native_pending IN (0,1));")?;
        }
        conn.execute_batch("BEGIN IMMEDIATE; DROP INDEX IF EXISTS one_active_job_per_app;\
             UPDATE jobs SET state='interrupted', detail='worker stopped before confirmation'\
             WHERE state IN ('running','cancelling');\
             UPDATE jobs SET native_pending=1 WHERE state='interrupted' AND backend IN ('ubuntu-deb','snap');\
             CREATE UNIQUE INDEX one_active_job_per_app ON jobs(app_id)\
             WHERE state IN ('queued','running','cancelling') OR\
               (state='interrupted' AND backend IN ('ubuntu-deb','snap')); COMMIT;")?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, Connection>, QueueError> {
        self.conn.lock().map_err(|_| QueueError::Poisoned)
    }

    pub fn enqueue(
        &self,
        app_id: &str,
        backend: Backend,
        action: Action,
    ) -> Result<Job, QueueError> {
        if !valid_app_id(app_id) {
            return Err(QueueError::InvalidAppId);
        }
        if matches!(backend, Backend::UbuntuDeb | Backend::Snap) && action == Action::Rollback {
            return Err(QueueError::InvalidTransition);
        }
        let job = Job {
            id: Uuid::new_v4().to_string(),
            app_id: app_id.into(),
            backend,
            action,
            state: JobState::Queued,
            detail: None,
            native_pending: false,
        };
        self.lock()?.execute(
            "INSERT INTO jobs(id,app_id,backend,action,state) VALUES (?1,?2,?3,?4,'queued')",
            params![job.id, job.app_id, backend.as_str(), action.as_str()],
        )?;
        Ok(job)
    }

    pub fn get(&self, id: &str) -> Result<Option<Job>, QueueError> {
        let conn = self.lock()?;
        let mut statement = conn.prepare(&format!("SELECT {COLUMNS} FROM jobs WHERE id=?1"))?;
        Ok(statement.query_row([id], row_job).optional()?)
    }

    pub fn list(&self) -> Result<Vec<Job>, QueueError> {
        let conn = self.lock()?;
        let mut statement =
            conn.prepare(&format!("SELECT {COLUMNS} FROM jobs ORDER BY sequence"))?;
        let rows = statement.query_map([], row_job)?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    pub fn list_recent(&self, limit: usize) -> Result<(Vec<Job>, bool), QueueError> {
        if !(1..=256).contains(&limit) {
            return Err(QueueError::InvalidTransition);
        }
        let conn = self.lock()?;
        let total: i64 = conn.query_row("SELECT COUNT(*) FROM jobs", [], |row| row.get(0))?;
        let mut statement = conn.prepare(&format!(
            "SELECT {COLUMNS} FROM jobs ORDER BY sequence DESC LIMIT ?1"
        ))?;
        let rows = statement.query_map([limit as i64], row_job)?;
        let mut recent = rows.collect::<rusqlite::Result<Vec<_>>>()?;
        recent.reverse();
        Ok((recent, total > limit as i64))
    }

    pub fn start_next(&self) -> Result<Option<Job>, QueueError> {
        let conn = self.lock()?;
        let mut statement = conn.prepare(&format!("UPDATE jobs SET state='running' WHERE sequence=(\
            SELECT sequence FROM jobs WHERE state='queued' ORDER BY sequence LIMIT 1) RETURNING {COLUMNS}"))?;
        Ok(statement.query_row([], row_job).optional()?)
    }

    pub fn cancel(&self, id: &str) -> Result<(), QueueError> {
        let count = self.lock()?.execute(
            "UPDATE jobs SET state=CASE state WHEN 'queued' THEN 'cancelled'\
            WHEN 'running' THEN 'cancelling' END WHERE id=?1 AND state IN ('queued','running') AND native_pending=0 AND NOT (state='running' AND backend IN ('ubuntu-deb','snap'))",
            [id],
        )?;
        if count == 1 {
            Ok(())
        } else {
            Err(QueueError::InvalidTransition)
        }
    }

    pub fn acknowledge_cancel(&self, id: &str) -> Result<(), QueueError> {
        let count = self.lock()?.execute(
            "UPDATE jobs SET state='cancelled' WHERE id=?1 AND state='cancelling'",
            [id],
        )?;
        if count == 1 {
            Ok(())
        } else {
            Err(QueueError::InvalidTransition)
        }
    }

    pub fn finish(&self, id: &str, success: bool, detail: Option<&str>) -> Result<(), QueueError> {
        if detail.is_some_and(|text| text.len() > 4096) {
            return Err(QueueError::InvalidTransition);
        }
        let state = if success { "succeeded" } else { "failed" };
        let count = self.lock()?.execute(
            "UPDATE jobs SET state=?2,detail=?3,native_pending=0 WHERE id=?1 AND state='running'",
            params![id, state, detail],
        )?;
        if count == 1 {
            Ok(())
        } else {
            Err(QueueError::InvalidTransition)
        }
    }

    pub fn finish_after_cancel(
        &self,
        id: &str,
        success: bool,
        detail: Option<&str>,
    ) -> Result<(), QueueError> {
        if detail.is_some_and(|text| text.len() > 4096) {
            return Err(QueueError::InvalidTransition);
        }
        let state = if success { "succeeded" } else { "failed" };
        let count = self.lock()?.execute(
            "UPDATE jobs SET state=?2,detail=?3 WHERE id=?1 AND state='cancelling'",
            params![id, state, detail],
        )?;
        if count == 1 {
            Ok(())
        } else {
            Err(QueueError::InvalidTransition)
        }
    }

    pub fn retry(&self, id: &str) -> Result<Job, QueueError> {
        let old = self.get(id)?.ok_or(QueueError::InvalidTransition)?;
        if matches!(old.backend, Backend::UbuntuDeb | Backend::Snap)
            && old.state == JobState::Interrupted
        {
            let count = self.lock()?.execute(
                "UPDATE jobs SET state='queued',detail=NULL WHERE id=?1 AND state='interrupted'",
                [id],
            )?;
            if count != 1 {
                return Err(QueueError::InvalidTransition);
            }
            return self.get(id)?.ok_or(QueueError::InvalidTransition);
        }
        if !matches!(
            old.state,
            JobState::Interrupted | JobState::Failed | JobState::Cancelled
        ) {
            return Err(QueueError::InvalidTransition);
        }
        self.enqueue(&old.app_id, old.backend, old.action)
    }

    /// Unknown native results retain their provider request ID and block new operations.
    pub fn interrupt(&self, id: &str, detail: &str) -> Result<(), QueueError> {
        if detail.len() > 4096 {
            return Err(QueueError::InvalidTransition);
        }
        let count = self.lock()?.execute("UPDATE jobs SET state='interrupted',detail=?2,native_pending=CASE WHEN backend IN ('ubuntu-deb','snap') THEN 1 ELSE 0 END WHERE id=?1 AND state IN ('running','cancelling')", params![id,detail])?;
        if count == 1 {
            Ok(())
        } else {
            Err(QueueError::InvalidTransition)
        }
    }
}

fn valid_app_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'.')
}
