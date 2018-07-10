use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub fn create_database(pool: &Pool<PostgresConnectionManager>){
    let conn = pool.clone().get().unwrap();
    let _ = conn.execute(r#"CREATE TABLE documents (
            id SERIAL PRIMARY KEY,
            correspondent INT,
            title text,
            "date" DATE,
            --ocr_result TEXT,
            --pages INT,
            hidden BOOL DEFAULT FALSE
        );"#, &[]);

    let _ = conn.execute(r#"CREATE TABLE pages (
        document_id INT REFERENCES documents(id),
        ocr_result TEXT,
        number INT,
        PRIMARY KEY(document_id, number)
    );"#, &[]);

    let _ = conn.execute(r#"CREATE TABLE tags (
        slug TEXT primary key,
        name TEXT, color TEXT
        );"#, &[]);

    let _ = conn.execute(r#"CREATE TABLE correspondents (
        id SERIAL primary key,
        name TEXT
        )"#, &[]);

    let _ = conn.execute(r#"CREATE TABLE tags_documents (
        tag_slug TEXT REFERENCES tags(slug),
        document_id INT REFERENCES documents(id),
        PRIMARY KEY(tag_slug, document_id)
        );"#, &[]);
}