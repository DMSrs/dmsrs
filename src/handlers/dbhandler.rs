use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub fn create_database(pool: &Pool<PostgresConnectionManager>){
    let conn = pool.clone().get().unwrap();

    if conn.query("SELECT value FROM settings WHERE key='db_version'", &[]).is_ok() {
        return;
    }

    conn.execute(r#"CREATE TABLE correspondents (
        id SERIAL primary key,
        name TEXT
    )"#, &[]).expect("Unable to create correspondents");

    conn.execute(r#"CREATE TABLE documents (
        id SERIAL PRIMARY KEY,
        correspondent INT REFERENCES correspondents(id),
        title text,
        added_on DATE,
        "date" DATE,
        sha256sum TEXT UNIQUE,
        hidden BOOL DEFAULT FALSE
    );"#, &[]).expect("Unable to create documents");

    conn.execute(r#"CREATE TABLE pages (
        document_id INT REFERENCES documents(id),
        ocr_result TEXT,
        number INT,
        PRIMARY KEY(document_id, number)
    );"#, &[]).expect("Unable to create pages");

    conn.execute(r#"CREATE TABLE tags (
        slug TEXT primary key,
        name TEXT, color TEXT
    );"#, &[]).expect("Unable to create tags");

    conn.execute(r#"CREATE TABLE tags_documents (
        tag_slug TEXT REFERENCES tags(slug),
        document_id INT REFERENCES documents(id),
        PRIMARY KEY(tag_slug, document_id)
    );"#, &[]).expect("Unable to create tags_documents");
    
    conn.execute(r#"CREATE TABLE settings (
        key TEXT PRIMARY KEY,
        value TEXT
    );"#, &[]).expect("Unable to create settings");

    conn.execute(r#"INSERT INTO settings VALUES (
        'db_version', '1');"#, &[]).expect("Unable to add db version");
}