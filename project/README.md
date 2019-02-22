# Rustbridge workshop project

We're going to build a webserver which will show a random compliment every time it's invoked.

## Dependencies (check crates.io)
* [handlebars](https://www.crates.io/crates/handlebars)
* [motivations](https://www.crates.io/crates/motivations)
* [pick-one](https://www.crates.io/crates/pick-one)
* [simple-server](https://www.crates.io/crates/simple-server)

## Instructions

**main.rs**

* Create a function get_server_port which returns a String
* Use env::var to return the value of PORT from the environment or return a default value of 7878
* Use Server::new() to define a variable app. This will require you to make a closure taking a request and response as an argument.
* In the closure define a variable with some basic HTML which we'll feed to the server in a moment. The server expects body content to be in bytes. The String object conviniently has an into_bytes() method to help us out.
* Make the closure return a positive Result (Ok) with a response.header (Content-Type text/html; charset=utf-8) and a body containing the HTML. The latter is a bit tougher. Scroll down for the answer if you're stuck.
* Define a string slice variable with the value "0.0.0.0"
* Define a variable port with the result of invoking `get_server_port()` as the value
* Print a message showing the address (host and port) the server is running on
* Use app.listen to start the server. Listen will take the host and port as an argument.

**templates.rs**

* Create the templates.rs file. In Rust files are considered to be modules. This will be the module containing the functionality to render a template
* Add `mod templates` to main.rs near the use statements
* Create a public function motivation() which returns a vector of type u8

**motivation()**

* Define a variable context with BTreeMap::new()
* Use pickone::pick_one_str to pick a random string from motivations::MOTIVATIONS
* Our crab images are conviniently called crab-1.jpg to crab-13.jpg. Define a variable crab which contains a random number (or string with a number) between 1 and 13.
* insert the motivation as key "motivation" and the crab as key "image" into the context btree
* Use Handlebars::new() to create a new instance of the handlebar template engine
* Use `handlebars.register_template_file` to link the context and to load "templates/motivation.html" to the template
* Use `handlebars.render` to render the "motivation" template. Tip: Vec<u8> represents bytes

**main.rs**

* Change the body to the result of invoking the templates::motivation() function you created earlier














Server::new with closure
```
let app = Server::new(|_request, mut response| {
    let html = String::from("<html><head><title>Rustbridge</title></head><body><h1>Hello Rust!</h1></body></html>").into_bytes();
    Ok(response.header("Content-Type", "text/html; charset=utf-8").body(html)?)
});
```