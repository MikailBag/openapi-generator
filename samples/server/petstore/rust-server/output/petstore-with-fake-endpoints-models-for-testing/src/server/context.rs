use crate::headers::*;
use std::io;
use std::marker::PhantomData;
use std::future::Future;
use std::pin::Pin;
use std::default::Default;
use futures::FutureExt;
use headers::{ContentType, HeaderMapExt};
use hyper;
use hyper::{Request, Response, Error, StatusCode};
use url::form_urlencoded;
use openapi_context::auth::{Authorization, AuthData, Scopes};
use openapi_context::{ContextualPayload, Has, Pop, Push, XSpanId};
use crate::Api;

/// Middleware to extract authentication data from request
pub struct AddContext<T, A>
{
    inner: T,
    marker: PhantomData<A>,
}

impl<T, A, B, C, D> AddContext<T, A>
    where
        A: Default + Push<XSpanId, Result=B>,
        B: Push<Option<AuthData>, Result=C>,
        C: Push<Option<Authorization>, Result=D>,
        T: hyper::service::Service<(Request<hyper::Body>, D), Response = Response<hyper::Body>, Error = Error>,
{
    pub fn new(inner: T) -> AddContext<T, A> {
        AddContext {
            inner,
            marker: PhantomData,
        }
    }
}

impl<T, A, B, C, D> hyper::service::Service<ContextualPayload<C>> for AddContext<T, A>
    where
        A: Default + Push<XSpanId, Result=B>,
        B: Push<Option<AuthData>, Result=C>,
        C: Push<Option<Authorization>, Result=D> + Send + Sync,
        T: hyper::service::Service<(Request<hyper::Body>, D), Response = Response<hyper::Body>, Error = Error>,
{
    type Response = Response<hyper::Body>;
    type Error = Error;
    type Future = T::Future;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: ContextualPayload<C>) -> Self::Future {
        let context = A::default().push(XSpanId::get_or_generate(&req.inner));
        let mut req = req.inner;

        {
            if let Some(ref header) = req.headers_mut().typed_get::<ApiKey1>(){
                let auth_data = AuthData::ApiKey(header.0.clone());
                let context = context.push(Some(auth_data));
                let context = context.push(None::<Authorization>);
                return self.inner.call((req, context));
            }
        }
        {
            let key = form_urlencoded::parse(req.query().unwrap_or_default().as_bytes())
                .filter(|e| e.0 == "api_key_query")
                .map(|e| e.1.clone().into_owned())
                .nth(0);
            if let Some(key) = key {
                let auth_data = AuthData::ApiKey(key);
                let context = context.push(Some(auth_data));
                let context = context.push(None::<Authorization>);
                return self.inner.call((req, context));
            }
        }
        {
            use headers::Authorization as HyperAuth;
            use headers::authorization::Basic;
            use std::ops::Deref;
            if let Some(ref basic) = req.headers_mut().typed_get::<HyperAuth<Basic>>() {
                let auth_data = AuthData::Basic(basic.0.clone());
                let context = context.push(Some(auth_data));
                let context = context.push(None::<Authorization>);
                return self.inner.call((req, context));
            }
        }
        {
            use headers::Authorization as HyperAuth;
            use headers::authorization::Bearer;
            use std::ops::Deref;
            if let Some(ref bearer) = req.headers_mut().typed_get::<HyperAuth<Bearer>>(){
                let auth_data = AuthData::Bearer(bearer.clone());
                let context = context.push(Some(auth_data));
                let context = context.push(None::<Authorization>);
                return self.inner.call((req, context));
            }
        }

        let context = context.push(None::<AuthData>);
        let context = context.push(None::<Authorization>);
        return self.inner.call((req, context));
    }
}
