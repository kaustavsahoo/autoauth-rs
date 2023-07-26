use minreq::Response;

const REQUEST_URL: &str = "http://172.16.1.3:8002/index.php?zone=lan";

pub fn login(user: &str, pass: &str) -> Response {
    let body_str = format!(
        "redirurl=https%3A%2F%2Fmnit.ac.in&zone=lan&auth_user={}&auth_pass={}&accept=LOGIN",
        user, pass
    );

    let response = minreq::post(REQUEST_URL)
        .with_body(body_str)
        .with_header("accept", r#"text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"#)
        .with_header("content-type", "application/x-www-form-urlencoded")
        .send()
        .expect("Could not send request");

    response
}
