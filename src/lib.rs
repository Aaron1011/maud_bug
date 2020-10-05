#[macro_use]
extern crate maud;


pub async fn serve() {
    // Render HTML using Maud templating engine
    let doc = html! {
        body."w-100" {
            #initial data-json=("") {}
        }
    };
}
