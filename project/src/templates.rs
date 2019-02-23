use std::collections::BTreeMap;
use motivations::MOTIVATIONS;
use pick_one::pick_one_str;
use handlebars::Handlebars;

pub fn motivation() -> Vec<u8> {
    let mut context = BTreeMap::new();
    let motivation =  pick_one_str(&MOTIVATIONS);
    let crab = pick_one_str(&["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"]);
    context.insert("motivation", motivation);
    context.insert("image", crab);

    let mut handlebar = Handlebars::new();
    handlebar.register_template_file("motivation", "templates/motivation.html").expect("Error loading template file");
    handlebar.render("motivation", &context).expect("Error rendering template").into_bytes()
}
