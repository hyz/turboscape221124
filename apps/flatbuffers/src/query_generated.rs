// automatically generated by the FlatBuffers compiler, do not modify

// @generated

use core::cmp::Ordering;
use core::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod query {

    use core::cmp::Ordering;
    use core::mem;

    extern crate flatbuffers;
    use self::flatbuffers::{EndianScalar, Follow};

    #[deprecated(
        since = "2.0.0",
        note = "Use associated constants instead. This will no longer be generated in 2021."
    )]
    pub const ENUM_MIN_METHOD: i8 = 0;
    #[deprecated(
        since = "2.0.0",
        note = "Use associated constants instead. This will no longer be generated in 2021."
    )]
    pub const ENUM_MAX_METHOD: i8 = 5;
    #[deprecated(
        since = "2.0.0",
        note = "Use associated constants instead. This will no longer be generated in 2021."
    )]
    #[allow(non_camel_case_types)]
    pub const ENUM_VALUES_METHOD: [Method; 6] = [
        Method::Method_,
        Method::Get,
        Method::Head,
        Method::Post,
        Method::Put,
        Method::Delete,
    ];

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    #[repr(transparent)]
    pub struct Method(pub i8);
    #[allow(non_upper_case_globals)]
    impl Method {
        pub const Method_: Self = Self(0);
        pub const Get: Self = Self(1);
        pub const Head: Self = Self(2);
        pub const Post: Self = Self(3);
        pub const Put: Self = Self(4);
        pub const Delete: Self = Self(5);

        pub const ENUM_MIN: i8 = 0;
        pub const ENUM_MAX: i8 = 5;
        pub const ENUM_VALUES: &'static [Self] = &[
            Self::Method_,
            Self::Get,
            Self::Head,
            Self::Post,
            Self::Put,
            Self::Delete,
        ];
        /// Returns the variant's name or "" if unknown.
        pub fn variant_name(self) -> Option<&'static str> {
            match self {
                Self::Method_ => Some("Method_"),
                Self::Get => Some("Get"),
                Self::Head => Some("Head"),
                Self::Post => Some("Post"),
                Self::Put => Some("Put"),
                Self::Delete => Some("Delete"),
                _ => None,
            }
        }
    }
    impl core::fmt::Debug for Method {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            if let Some(name) = self.variant_name() {
                f.write_str(name)
            } else {
                f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
            }
        }
    }
    impl<'a> flatbuffers::Follow<'a> for Method {
        type Inner = Self;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
            Self(b)
        }
    }

    impl flatbuffers::Push for Method {
        type Output = Method;
        #[inline]
        unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
            flatbuffers::emplace_scalar::<i8>(dst, self.0);
        }
    }

    impl flatbuffers::EndianScalar for Method {
        type Scalar = i8;
        #[inline]
        fn to_little_endian(self) -> i8 {
            self.0.to_le()
        }
        #[inline]
        #[allow(clippy::wrong_self_convention)]
        fn from_little_endian(v: i8) -> Self {
            let b = i8::from_le(v);
            Self(b)
        }
    }

    impl<'a> flatbuffers::Verifiable for Method {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            i8::run_verifier(v, pos)
        }
    }

    impl flatbuffers::SimpleToVerifyInSlice for Method {}
    pub enum PairOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct Pair<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Pair<'a> {
        type Inner = Pair<'a>;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table::new(buf, loc),
            }
        }
    }

    impl<'a> Pair<'a> {
        pub const VT_KEY: flatbuffers::VOffsetT = 4;
        pub const VT_VALUE: flatbuffers::VOffsetT = 6;

        #[inline]
        pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Pair { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args PairArgs<'args>,
        ) -> flatbuffers::WIPOffset<Pair<'bldr>> {
            let mut builder = PairBuilder::new(_fbb);
            if let Some(x) = args.value {
                builder.add_value(x);
            }
            if let Some(x) = args.key {
                builder.add_key(x);
            }
            builder.finish()
        }

        #[inline]
        pub fn key(&self) -> Option<&'a str> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<&str>>(Pair::VT_KEY, None)
            }
        }
        #[inline]
        pub fn value(&self) -> Option<&'a str> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<&str>>(Pair::VT_VALUE, None)
            }
        }
    }

    impl flatbuffers::Verifiable for Pair<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<flatbuffers::ForwardsUOffset<&str>>("key", Self::VT_KEY, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<&str>>("value", Self::VT_VALUE, false)?
                .finish();
            Ok(())
        }
    }
    pub struct PairArgs<'a> {
        pub key: Option<flatbuffers::WIPOffset<&'a str>>,
        pub value: Option<flatbuffers::WIPOffset<&'a str>>,
    }
    impl<'a> Default for PairArgs<'a> {
        #[inline]
        fn default() -> Self {
            PairArgs {
                key: None,
                value: None,
            }
        }
    }

    pub struct PairBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> PairBuilder<'a, 'b> {
        #[inline]
        pub fn add_key(&mut self, key: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Pair::VT_KEY, key);
        }
        #[inline]
        pub fn add_value(&mut self, value: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Pair::VT_VALUE, value);
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PairBuilder<'a, 'b> {
            let start = _fbb.start_table();
            PairBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Pair<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for Pair<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("Pair");
            ds.field("key", &self.key());
            ds.field("value", &self.value());
            ds.finish()
        }
    }
    pub enum RequestOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct Request<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Request<'a> {
        type Inner = Request<'a>;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table::new(buf, loc),
            }
        }
    }

    impl<'a> Request<'a> {
        pub const VT_METHOD: flatbuffers::VOffsetT = 4;
        pub const VT_URL: flatbuffers::VOffsetT = 6;
        pub const VT_BODY: flatbuffers::VOffsetT = 8;
        pub const VT_HEADERS: flatbuffers::VOffsetT = 10;

        #[inline]
        pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Request { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args RequestArgs<'args>,
        ) -> flatbuffers::WIPOffset<Request<'bldr>> {
            let mut builder = RequestBuilder::new(_fbb);
            if let Some(x) = args.headers {
                builder.add_headers(x);
            }
            if let Some(x) = args.body {
                builder.add_body(x);
            }
            if let Some(x) = args.url {
                builder.add_url(x);
            }
            builder.add_method(args.method);
            builder.finish()
        }

        #[inline]
        pub fn method(&self) -> Method {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<Method>(Request::VT_METHOD, Some(Method::Method_))
                    .unwrap()
            }
        }
        #[inline]
        pub fn url(&self) -> Option<&'a str> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<&str>>(Request::VT_URL, None)
            }
        }
        #[inline]
        pub fn body(&self) -> Option<flatbuffers::Vector<'a, u8>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                        Request::VT_BODY,
                        None,
                    )
            }
        }
        #[inline]
        pub fn headers(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pair<'a>>>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab.get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pair>>,
                >>(Request::VT_HEADERS, None)
            }
        }
    }

    impl flatbuffers::Verifiable for Request<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<Method>("method", Self::VT_METHOD, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<&str>>("url", Self::VT_URL, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                    "body",
                    Self::VT_BODY,
                    false,
                )?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Pair>>,
                >>("headers", Self::VT_HEADERS, false)?
                .finish();
            Ok(())
        }
    }
    pub struct RequestArgs<'a> {
        pub method: Method,
        pub url: Option<flatbuffers::WIPOffset<&'a str>>,
        pub body: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
        pub headers: Option<
            flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pair<'a>>>>,
        >,
    }
    impl<'a> Default for RequestArgs<'a> {
        #[inline]
        fn default() -> Self {
            RequestArgs {
                method: Method::Method_,
                url: None,
                body: None,
                headers: None,
            }
        }
    }

    pub struct RequestBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> RequestBuilder<'a, 'b> {
        #[inline]
        pub fn add_method(&mut self, method: Method) {
            self.fbb_
                .push_slot::<Method>(Request::VT_METHOD, method, Method::Method_);
        }
        #[inline]
        pub fn add_url(&mut self, url: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Request::VT_URL, url);
        }
        #[inline]
        pub fn add_body(&mut self, body: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Request::VT_BODY, body);
        }
        #[inline]
        pub fn add_headers(
            &mut self,
            headers: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Pair<'b>>>,
            >,
        ) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Request::VT_HEADERS, headers);
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RequestBuilder<'a, 'b> {
            let start = _fbb.start_table();
            RequestBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Request<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for Request<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("Request");
            ds.field("method", &self.method());
            ds.field("url", &self.url());
            ds.field("body", &self.body());
            ds.field("headers", &self.headers());
            ds.finish()
        }
    }
    pub enum ResponseOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct Response<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Response<'a> {
        type Inner = Response<'a>;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table::new(buf, loc),
            }
        }
    }

    impl<'a> Response<'a> {
        pub const VT_STATUS: flatbuffers::VOffsetT = 4;
        pub const VT_CONTENT: flatbuffers::VOffsetT = 6;
        pub const VT_CONTENTTYPE: flatbuffers::VOffsetT = 8;
        pub const VT_HEADERS: flatbuffers::VOffsetT = 10;

        #[inline]
        pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Response { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args ResponseArgs<'args>,
        ) -> flatbuffers::WIPOffset<Response<'bldr>> {
            let mut builder = ResponseBuilder::new(_fbb);
            if let Some(x) = args.headers {
                builder.add_headers(x);
            }
            if let Some(x) = args.contenttype {
                builder.add_contenttype(x);
            }
            if let Some(x) = args.content {
                builder.add_content(x);
            }
            builder.add_status(args.status);
            builder.finish()
        }

        #[inline]
        pub fn status(&self) -> i16 {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe { self._tab.get::<i16>(Response::VT_STATUS, Some(0)).unwrap() }
        }
        #[inline]
        pub fn content(&self) -> Option<flatbuffers::Vector<'a, u8>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                        Response::VT_CONTENT,
                        None,
                    )
            }
        }
        #[inline]
        pub fn contenttype(&self) -> Option<&'a str> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<&str>>(Response::VT_CONTENTTYPE, None)
            }
        }
        #[inline]
        pub fn headers(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pair<'a>>>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab.get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pair>>,
                >>(Response::VT_HEADERS, None)
            }
        }
    }

    impl flatbuffers::Verifiable for Response<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<i16>("status", Self::VT_STATUS, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                    "content",
                    Self::VT_CONTENT,
                    false,
                )?
                .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                    "contenttype",
                    Self::VT_CONTENTTYPE,
                    false,
                )?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Pair>>,
                >>("headers", Self::VT_HEADERS, false)?
                .finish();
            Ok(())
        }
    }
    pub struct ResponseArgs<'a> {
        pub status: i16,
        pub content: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
        pub contenttype: Option<flatbuffers::WIPOffset<&'a str>>,
        pub headers: Option<
            flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pair<'a>>>>,
        >,
    }
    impl<'a> Default for ResponseArgs<'a> {
        #[inline]
        fn default() -> Self {
            ResponseArgs {
                status: 0,
                content: None,
                contenttype: None,
                headers: None,
            }
        }
    }

    pub struct ResponseBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> ResponseBuilder<'a, 'b> {
        #[inline]
        pub fn add_status(&mut self, status: i16) {
            self.fbb_.push_slot::<i16>(Response::VT_STATUS, status, 0);
        }
        #[inline]
        pub fn add_content(
            &mut self,
            content: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
        ) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Response::VT_CONTENT, content);
        }
        #[inline]
        pub fn add_contenttype(&mut self, contenttype: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                Response::VT_CONTENTTYPE,
                contenttype,
            );
        }
        #[inline]
        pub fn add_headers(
            &mut self,
            headers: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Pair<'b>>>,
            >,
        ) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Response::VT_HEADERS, headers);
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ResponseBuilder<'a, 'b> {
            let start = _fbb.start_table();
            ResponseBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Response<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for Response<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("Response");
            ds.field("status", &self.status());
            ds.field("content", &self.content());
            ds.field("contenttype", &self.contenttype());
            ds.field("headers", &self.headers());
            ds.finish()
        }
    }
    pub enum QueryOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct Query<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Query<'a> {
        type Inner = Query<'a>;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table::new(buf, loc),
            }
        }
    }

    impl<'a> Query<'a> {
        pub const VT_CONTEXT: flatbuffers::VOffsetT = 4;
        pub const VT_REQUEST: flatbuffers::VOffsetT = 6;
        pub const VT_RESPONSE: flatbuffers::VOffsetT = 8;

        #[inline]
        pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Query { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args QueryArgs<'args>,
        ) -> flatbuffers::WIPOffset<Query<'bldr>> {
            let mut builder = QueryBuilder::new(_fbb);
            if let Some(x) = args.response {
                builder.add_response(x);
            }
            if let Some(x) = args.request {
                builder.add_request(x);
            }
            if let Some(x) = args.context {
                builder.add_context(x);
            }
            builder.finish()
        }

        #[inline]
        pub fn context(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pair<'a>>>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab.get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pair>>,
                >>(Query::VT_CONTEXT, None)
            }
        }
        #[inline]
        pub fn request(&self) -> Option<Request<'a>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<Request>>(Query::VT_REQUEST, None)
            }
        }
        #[inline]
        pub fn response(&self) -> Option<Response<'a>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<Response>>(Query::VT_RESPONSE, None)
            }
        }
    }

    impl flatbuffers::Verifiable for Query<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Pair>>,
                >>("context", Self::VT_CONTEXT, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<Request>>(
                    "request",
                    Self::VT_REQUEST,
                    false,
                )?
                .visit_field::<flatbuffers::ForwardsUOffset<Response>>(
                    "response",
                    Self::VT_RESPONSE,
                    false,
                )?
                .finish();
            Ok(())
        }
    }
    pub struct QueryArgs<'a> {
        pub context: Option<
            flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pair<'a>>>>,
        >,
        pub request: Option<flatbuffers::WIPOffset<Request<'a>>>,
        pub response: Option<flatbuffers::WIPOffset<Response<'a>>>,
    }
    impl<'a> Default for QueryArgs<'a> {
        #[inline]
        fn default() -> Self {
            QueryArgs {
                context: None,
                request: None,
                response: None,
            }
        }
    }

    pub struct QueryBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> QueryBuilder<'a, 'b> {
        #[inline]
        pub fn add_context(
            &mut self,
            context: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Pair<'b>>>,
            >,
        ) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Query::VT_CONTEXT, context);
        }
        #[inline]
        pub fn add_request(&mut self, request: flatbuffers::WIPOffset<Request<'b>>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<Request>>(Query::VT_REQUEST, request);
        }
        #[inline]
        pub fn add_response(&mut self, response: flatbuffers::WIPOffset<Response<'b>>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<Response>>(Query::VT_RESPONSE, response);
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> QueryBuilder<'a, 'b> {
            let start = _fbb.start_table();
            QueryBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Query<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for Query<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("Query");
            ds.field("context", &self.context());
            ds.field("request", &self.request());
            ds.field("response", &self.response());
            ds.finish()
        }
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a `Query`
    /// and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_query_unchecked`.
    pub fn root_as_query(buf: &[u8]) -> Result<Query, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root::<Query>(buf)
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a size prefixed
    /// `Query` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `size_prefixed_root_as_query_unchecked`.
    pub fn size_prefixed_root_as_query(
        buf: &[u8],
    ) -> Result<Query, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root::<Query>(buf)
    }
    #[inline]
    /// Verifies, with the given options, that a buffer of bytes
    /// contains a `Query` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_query_unchecked`.
    pub fn root_as_query_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<Query<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root_with_opts::<Query<'b>>(opts, buf)
    }
    #[inline]
    /// Verifies, with the given verifier options, that a buffer of
    /// bytes contains a size prefixed `Query` and returns
    /// it. Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_query_unchecked`.
    pub fn size_prefixed_root_as_query_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<Query<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root_with_opts::<Query<'b>>(opts, buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a Query and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid `Query`.
    pub unsafe fn root_as_query_unchecked(buf: &[u8]) -> Query {
        flatbuffers::root_unchecked::<Query>(buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a size prefixed Query and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid size prefixed `Query`.
    pub unsafe fn size_prefixed_root_as_query_unchecked(buf: &[u8]) -> Query {
        flatbuffers::size_prefixed_root_unchecked::<Query>(buf)
    }
    #[inline]
    pub fn finish_query_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<Query<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_query_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<Query<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod query
