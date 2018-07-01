use rocket::State;
use routes::RoutesHandler;
use rocket_contrib::Template;
use tera::Context;
use models::document::Document;
use models::correspondent::Correspondent;
use chrono::Utc;
use models::picture::Picture;
use models::tag::Tag;

use handlers::documenthandler::fetch_documents;

#[get("/")]
pub fn index(rh: State<RoutesHandler>) -> Template {

    let mut context = Context::new();
    let mut documents : Vec<Document> = Vec::new();
    /*documents.push(Document {
        title: String::from("Credit Card Statement"),
        from: Correspondent {
            id: 1,
            name: "Postfinance".to_string()
        },
        date: Utc::now(),
        image: Picture {
            src: "img/demo/document-1.jpg".to_string(),
        },
        tags: vec![Tag{
            name: "Credit Card".to_string(),
            slug: "credit-card".to_string(),
            color: "#FFB74D".to_string()
        }, Tag{
            name: "Finance".to_string(),
            slug: "finance".to_string(),
            color: "#EF6C00".to_string()
        }],
    });

    documents.push(Document {
        title: String::from("Results"),
        date: Utc::now(),
        from: Correspondent {
            id: 2,
            name: "Scuola Universitaria Professionale della Svizzera Italiana".to_string()
        },
        image: Picture {
            src: "img/demo/document-1.jpg".to_string(),
        },
        tags: vec![Tag{
            name: "School".to_string(),
            slug: "school".to_string(),
            color: "#00695C".to_string()
        }, Tag{
            name: "Personal".to_string(),
            slug: "personal".to_string(),
            color: "#9C27B0".to_string()
        }, Tag{
            name: "Urgent".to_string(),
            slug: "urgent".to_string(),
            color: "#e53935".to_string()
        }, Tag{
            name: "Education".to_string(),
            slug: "education".to_string(),
            color: "#1565C0".to_string()
        }
        ],
    });*/

    let documents = fetch_documents(&rh.pool);

    context.add("documents", &documents);

    Template::render("documents", &context)
}