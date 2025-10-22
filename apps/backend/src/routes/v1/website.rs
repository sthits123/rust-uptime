use axum::{Router};
use repositories::{create_website,get_websites,get_website_by_id};

Router::new().route("/monitors", post(create_website));
Router::new().route("/monitors",get(get_websites)));
Roueter::new().route("/monotirs/:id",get(get_website_by_id()));
