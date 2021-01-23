use askama::Template;

#[derive(Template)]
#[template(path = "click.html")]
struct ClickTemplate {
    target: &'static str,
    down_up: &'static str,
}

pub async fn click(name: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let target = match name.as_str() {
        "mouseleft" => "mouseleft",
        _ => {
            return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
        }
    };

    Ok(Box::new(ClickTemplate {
        target,
        down_up: "false"
    }))
}
