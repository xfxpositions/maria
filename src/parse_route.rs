pub fn parse_route(request: String, routes: Vec<&str>) -> String {
    let first_line = request.lines().next().unwrap();
    let mut parts = first_line.splitn(3, ' ');

    let http_method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let route = Route {
        method: http_method.to_string(),
        path: path.to_string(),
    };
    let mut response = String::from("");
    for path in routes.iter() {
        println!(
            "incoming path: {}, our path: {}",
            route.path,
            path.to_string()
        );
        if route.path == path.to_string() {
            let message = String::from("Hello world!");
            response = format!(
                "HTTP/1.1 200 OK\nContent-Length: {}\nContent-Type: text/plain\n\n{}",
                message.len(),
                message
            );
        } else {
            let message = String::from("page is not found :(");
            response = format!(
                "HTTP/1.1 404 NOT FOUND\nContent-Length: {}\nContent-Type: text/plain\n\n{}",
                message.len(),
                message
            );
        }
    }
    println!("HTTP method: {}", http_method);
    println!("Route: {:?}", route);
    return response;
}
