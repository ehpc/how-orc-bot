use futures::future::BoxFuture;
use std::{fmt::Debug, sync::Arc};

pub struct TracingErrorHandler {
    text: String,
}

impl TracingErrorHandler {
    pub fn with_custom_text<T>(text: T) -> Arc<Self>
    where
        T: Into<String>,
    {
        Arc::new(Self { text: text.into() })
    }
}

impl<E> teloxide::error_handlers::ErrorHandler<E> for TracingErrorHandler
where
    E: Debug,
{
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()> {
        tracing::error!("{text}: {:?}", error, text = self.text);
        Box::pin(async {})
    }
}
