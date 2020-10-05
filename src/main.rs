use std::future::Future;

fn mk_ctxt() -> std::task::Context<'static> {
    panic!()
}

fn main() {
    Box::pin(maud_bug::serve()).as_mut().poll(&mut mk_ctxt());
}
