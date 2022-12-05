//! Protocol Buffer extractor and response.

use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::{rejection::BytesRejection, FromRequest},
    response::{IntoResponse, Response},
    BoxError,
};
use bytes::BytesMut;
use flatbuffers::{Follow, ForwardsUOffset, Verifiable, Verifier, VerifierOptions};
use http::{Request, StatusCode};
// use prost::Message;
use std::{
    error::Error,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::protocols::query_generated::query::Query;

/// A Protocol Buffer message extractor and response.
///
/// This can be used both as an extractor and as a response.
///
/// # As extractor
///
/// When used as an extractor, it can decode request bodies into some type that
/// implements [`prost::Message`]. The request will be rejected (and a [`FlatbufRejection`] will
/// be returned) if:
///
/// - The body couldn't be decoded into the target Protocol Buffer message type.
/// - Buffering the request body fails.
///
/// See [`FlatbufRejection`] for more details.
///
/// The extractor does not expect a `Content-Type` header to be present in the request.
///
/// # Extractor example
///
/// ```rust,no_run
/// use axum::{routing::post, Router};
/// use axum_extra::protobuf::Flatbuf;
///
/// #[derive(prost::Message)]
/// struct CreateUser {
///     #[prost(string, tag="1")]
///     email: String,
///     #[prost(string, tag="2")]
///     password: String,
/// }
///
/// async fn create_user(Flatbuf(payload): Flatbuf<CreateUser>) {
///     // payload is `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # async {
/// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
/// # };
/// ```
///
/// # As response
///
/// When used as a response, it can encode any type that implements [`prost::Message`] to
/// a newly allocated buffer.
///
/// If no `Content-Type` header is set, the `Content-Type: application/octet-stream` header
/// will be used automatically.
///
/// # Response example
///
/// ```
/// use axum::{
///     extract::Path,
///     routing::get,
///     Router,
/// };
/// use axum_extra::protobuf::Flatbuf;
///
/// #[derive(prost::Message)]
/// struct User {
///     #[prost(string, tag="1")]
///     username: String,
/// }
///
/// async fn get_user(Path(user_id) : Path<String>) -> Flatbuf<User> {
///     let user = find_user(user_id).await;
///     Flatbuf(user)
/// }
///
/// async fn find_user(user_id: String) -> User {
///     // ...
///     # unimplemented!()
/// }
///
/// let app = Router::new().route("/users/:id", get(get_user));
/// # async {
/// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
/// # };
/// ```
//#[cfg_attr(docsrs, doc(cfg(feature = "flatbuf")))]
#[derive(Debug, Clone, Default)]
pub struct Flatbuf<T> {
    bytes: Vec<u8>,
    head: usize,
    // root: T,
    _marker: PhantomData<T>,
}

impl<T> Flatbuf<T> {
    fn new(bytes: Vec<u8>, head: usize) -> Self {
        Self {
            bytes,
            head,
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<'t, T, S, B> FromRequest<S, B> for Flatbuf<T>
where
    T: 't + Follow<'t> + Verifiable, //Copy + Clone + PartialEq, // T: Message + Default,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = FlatbufRejection;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        fn run_verifier<T: Verifiable>(data: &[u8]) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            // let _x = flatbuffers::root::<'t, T>(&bytes)?;
            let opts = VerifierOptions::default();
            let mut v = Verifier::new(&opts, data);
            <ForwardsUOffset<T>>::run_verifier(&mut v, 0)
        }

        let bytes = Bytes::from_request(req, state).await?;
        let bytes = Vec::from(bytes); // bytes.to_vec;

        let fb = Flatbuf::<T>::new(bytes, 0);
        run_verifier::<T>(&fb.bytes)?;

        Ok(fb)
    }
}

pub fn construct<'buf, T: Follow<'buf> + Verifiable>(
    fbuf: &'buf Flatbuf<T>,
) -> <T as Follow<'_>>::Inner {
    flatbuffers::root::<T>(&fbuf.bytes).unwrap()
}

// impl<'t, T: Sized + Follow<'t> + Verifiable> Deref for Flatbuf {
//     type Target = T;
//     fn deref(&'t self) -> Self::Target {
//         let t = flatbuffers::root::<T>(&self.0);
//         t.unwrap()
//     }
// }

impl<T> From<flatbuffers::FlatBufferBuilder<'_>> for Flatbuf<T> {
    fn from(inner: flatbuffers::FlatBufferBuilder) -> Self {
        _ = inner.finished_data();
        let (bytes, head) = inner.collapse();
        // buf.rotate_left(head);
        // buf.buf.truncate(buf.len() - head); //.resize(, 0);
        Self::new(bytes, head)
    }
}

impl<T> IntoResponse for Flatbuf<T>
// where T: Copy + Clone + PartialEq, //T: Message + Default,
{
    fn into_response(mut self) -> Response {
        // let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        // let mut buf = BytesMut::with_capacity(self.bytes.len() - self.head);
        let _ = self.bytes.drain(..self.head);
        let Flatbuf { bytes, .. } = self;
        // buf.copy_from_slice(&self.bytes[self.head..]);
        bytes.into_response()
        // match &self.0.encode(&mut buf) {
        //     Ok(()) => buf.into_response(),
        //     Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        // }
    }
}

/// Rejection type for [`Flatbuf`].
///
/// This rejection is used if the request body couldn't be decoded into the target type.
#[derive(Debug)]
pub struct FlatbufDecodeError(pub(crate) axum::Error);

impl FlatbufDecodeError {
    pub(crate) fn from_err<E>(err: E) -> Self
    where
        E: Into<axum::BoxError>,
    {
        Self(axum::Error::new(err))
    }
}

impl std::fmt::Display for FlatbufDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to decode the body: {:?}", self.0)
    }
}

impl std::error::Error for FlatbufDecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl IntoResponse for FlatbufDecodeError {
    fn into_response(self) -> Response {
        StatusCode::UNPROCESSABLE_ENTITY.into_response()
    }
}

/// Rejection used for [`Flatbuf`].
///
/// Contains one variant for each way the [`Flatbuf`] extractor
/// can fail.
#[derive(Debug)]
#[non_exhaustive]
pub enum FlatbufRejection {
    #[allow(missing_docs)]
    FlatbufDecodeError(FlatbufDecodeError),
    #[allow(missing_docs)]
    BytesRejection(BytesRejection),
    #[allow(missing_docs)]
    InvalidFlatbuffer(flatbuffers::InvalidFlatbuffer),
}

impl From<FlatbufDecodeError> for FlatbufRejection {
    fn from(inner: FlatbufDecodeError) -> Self {
        Self::FlatbufDecodeError(inner)
    }
}

impl From<BytesRejection> for FlatbufRejection {
    fn from(inner: BytesRejection) -> Self {
        Self::BytesRejection(inner)
    }
}
impl From<flatbuffers::InvalidFlatbuffer> for FlatbufRejection {
    fn from(inner: flatbuffers::InvalidFlatbuffer) -> Self {
        Self::InvalidFlatbuffer(inner)
    }
}

impl IntoResponse for FlatbufRejection {
    fn into_response(self) -> Response {
        match self {
            Self::FlatbufDecodeError(inner) => inner.into_response(),
            Self::BytesRejection(inner) => inner.into_response(),
            Self::InvalidFlatbuffer(inner) => inner.to_string().into_response(),
        }
    }
}
