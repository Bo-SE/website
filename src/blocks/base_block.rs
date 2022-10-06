use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

pub struct Block {
    url: String
}

impl Block {
    fn render<C>(&self, context: C) -> Template
        where C: Serialize {
        Template::render(self.url.clone(), context)
    }
}