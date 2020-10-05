pub async fn serve() {
    maud::html! {
        body."w-100" {
            #initial data-json=("") {}
        }
    };
}
