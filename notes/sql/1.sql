CREATE TABLE documents (id SERIAL PRIMARY KEY, correspondent INT, title text, "date" DATE, ocr_result TEXT, pages INT, hidden BOOL DEFAULT FALSE);
create table tags (slug TEXT primary key, name TEXT, color TEXT);
drop table tags;
create table correspondents (id SERIAL primary key, name TEXT);
drop table tags_documents;
create table tags_documents(tag_slug  TEXT references tags(slug), document_id INT references documents(id), primary KEY(tag_slug, document_id));

insert into tags values ('credit-card', 'Credit Card', 'FFB74D');
insert into tags values ('finance', 'Finance', 'EF6C00'), ('school', 'School', '00695C'), ('personal', 'Personal', '9C27B0'), ('urgent', 'Urgent','E53935'), ('education', 'Education', '1565C0');
insert into correspondents values (1, 'Postfinance'), (2, 'Scuola Universitaria Professionale Svizzera Italiana');
insert into documents values (1, 1, 'Credit Card Statement', '2018-07-01', '', 2, false);
insert into documents values (2, 2, 'Results', '2018-07-01', '', 1, false);
insert into tags_documents values ( 'credit-card', 1), ('finance', 1), ('school', 2), ( 'personal', 2), ('urgent', 2), ('education', 2);

SELECT
  title,
  correspondents.id as from_id,
  correspondents.name as from_name,
  "date",
  pages
FROM documents
INNER JOIN correspondents ON correspondents.id=documents.correspondent
WHERE documents.hidden = false

SELECT
  tags.slug,
  tags.name,
  tags.color
  FROM tags_documents
  INNER JOIN tags ON tags_documents.tag_slug = tags.slug
  WHERE tags_documents.document_id = 1