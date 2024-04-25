mod responses;
mod body;
mod headers;

use super::*;
pub use responses::*;
pub use body::*;
pub use headers::*;

/// Future wrapper for the request's Response.
///
/// Used by the `API` trait.
pub(super) struct ResFuture {
    pub handler: Pin<Box<dyn Future<Output=Result<Res, Res>> + Send>>,
}

impl Future for ResFuture {
    type Output = Result<Res, Res>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut pinned = std::pin::pin!(&mut self.handler);
        pinned.as_mut().poll(cx)
    }
}

/// Trait implemented for every API endpoint handler.
///
/// More functionality might come, but IDK.
pub(super) trait API where Self: 'static {
    /// Creates a handler future that will return a Response to the Request.
    ///
    /// The future is wrapped with `ResFuture` struct as
    /// when trying to return a raw future the compiler bitched around.
    fn handle(&self, meta: Parts, body: Incoming, addr: SocketAddr) -> ResFuture;

    /// Converts itself into trait object.
    fn into_obj(self) -> Box<dyn API + Sync> where Self: Sized, Self: Sync {
        Box::new(self)
    }
}
