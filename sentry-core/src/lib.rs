//! <p style="margin: -10px 0 0 15px; padding: 0; float: right;">
//!   <a href="https://sentry.io/"><img
//!     src="https://sentry-brand.storage.googleapis.com/sentry-logo-black.png"
//!     style="width: 260px"></a>
//! </p>
//!
//! This crate provides support for logging events and errors / panics to the
//! [Sentry](https://sentry.io/) error logging service.  It integrates with the standard panic
//! system in Rust as well as a few popular error handling setups.
//!
//! # Quickstart
//!
//! To use the crate you need to create a client first.  When a client is created it's typically
//! bound to the current thread by calling `bind_client`.  By default this happens by using the
//! `sentry::init` convenience function.  When the client is bound to the main thread it also
//! becomes the default client for future threads created but it is always possible to override the
//! client for a thread later by explicitly binding it.
//!
//! The `sentry::init` function returns a guard that when dropped will flush Events that were not
//! yet sent to the sentry service.  It has a two second deadline for this so shutdown of
//! applications might slightly delay as a result of this.  Keep the guard around or sending events
//! will not work.
//!
//! ```ignore
//! # use sentry_core as sentry;
//! let _guard = sentry::init("https://key@sentry.io/42");
//! sentry::capture_message("Hello World!", sentry::Level::Info);
//! // when the guard goes out of scope here, the client will wait up to two
//! // seconds to send remaining events to the service.
//! ```
//!
//! # Integrations
//!
//! What makes this crate useful are the various integrations that exist.  Some of them are enabled
//! by default, some uncommon ones or for deprecated parts of the ecosystem a feature flag needs to
//! be enabled.  For the available integrations and how to use them see
//! [integrations](integrations/index.html).
//!
//! # Scopes, Threads and Hubs
//!
//! Data is typically bound to a [`Scope`](struct.Scope.html).  Scopes are stored in a hidden stack
//! on a [`Hub`](struct.Hub.html).  Once the library has been initialized a hub is automatically
//! available.  In the default config a new hub is created for each thread and they act
//! independently.
//!
//! The thread that calls `sentry::init` initializes the first hub which then automatically becomes
//! the base of new hubs (You can get that hub by calling `Hub::main()`).  If a new thread is
//! spawned it gets a new hub based on that one (the thread calls `Hub::new_from_top(Hub::main())`).
//! The current thread's hub is returned from `Hub::current()`.  Any hub that is wrapped in an `Arc`
//! can be temporarily bound to a thread with `Hub::run`.  For more information see
//! [`Hub`](struct.Hub.html).
//!
//! Users are expected to reconfigure the scope with [`configure_scope`](fn.configure_scope.html).
//! For more elaborate scope management the hub needs to be interfaced with directly.
//!
//! In some situations (particularly in async code) it's often not possible to use the thread local
//! hub.  In that case a hub can be explicitly created and passed around.  However due to the nature
//! of some integrations some functionality like automatic breadcrumb recording depends on the
//! thread local hub being correctly configured.
//!
//! # Minimal API
//!
//! This crate can also be used in "minimal" mode.  This is enabled by disabling all default
//! features of the crate.  In that mode a minimal API set is retained that can be used to
//! instrument code for Sentry without actually using Sentry.  The minimal API is a small set of
//! APIs that dispatch to the underlying implementations on the configured Sentry client.  If the
//! client is not there the minimal API will blackhole a lot of operations.
//!
//! Only if a user then also uses and configures Sentry this code becomes used.
//!
//! In minimal mode some types are restricted in functionality.  For instance the `Client` is not
//! available and the `Hub` does not retain all API functionality. To see what the APIs in mnimal
//! mode look like you can build the docs for this crate without any features enabled.
//!
//! # Features
//!
//! Functionality of the crate can be turned on and off by feature flags.  This is the current list
//! of feature flags:
//!
//! Default features:
//!
//! * `with_client_implementation`: Turns on the real client implementation.
//! * `with_backtrace`: Enables backtrace support (automatically turned on in a few cases).
//! * `with_device_info`: Enables the device info context.
//! * `with_rust_info`: Enables the rust compiler info context instead of printing to stderr when
//!   debug is enabled on the hub.
//!
//! Additional context:
//!
//! * `with_debug_meta`: Adds debug meta to reported events (permits server side symbolication).
//!
//! Additional integrations:
//!
//! * `with_debug_to_log`: When enabled sentry will debug log to a debug log at all times.
//!
//! Testing:
//!
//! * `with_test_support`: Enables the test support module.
#![warn(missing_docs)]

#[macro_use]
mod macros;

mod api;
mod breadcrumbs;
mod hub;
mod scope;

pub mod integrations;

// NOTE: temporarily public, will move to `sentry-backtrace` soon
#[cfg(feature = "with_backtrace")]
pub mod backtrace_support;

#[cfg(feature = "with_client_implementation")]
mod client;
mod clientoptions;
mod constants;
mod intodsn;
mod transport;
#[cfg(feature = "with_client_implementation")]
pub mod utils;

#[cfg(any(test, feature = "with_test_support"))]
pub mod test;

/// Useful internals.
///
/// This module contains types that users of the crate typically do not
/// have to interface with directly.  These are often returned
/// from methods on other types.
pub mod internals {
    pub use crate::breadcrumbs::IntoBreadcrumbs;
    pub use crate::scope::ScopeGuard;

    pub use crate::intodsn::IntoDsn;
    pub use crate::transport::{Transport, TransportFactory};

    pub use sentry_types::{
        Auth, ChronoParseError, DateTime, DebugId, Dsn, ParseDebugIdError, ParseDsnError,
        ParseProjectIdError, ProjectId, Scheme, TimeZone, Utc, Uuid, UuidVariant, UuidVersion,
    };
}

// public api or exports from this crate
pub use crate::api::*;
pub use crate::clientoptions::ClientOptions;
pub use crate::hub::Hub;
pub use crate::integrations::Integration;
pub use crate::scope::Scope;

#[cfg(feature = "with_client_implementation")]
pub use crate::client::Client;

// public api from other crates
pub use sentry_types::protocol::v7 as protocol;
pub use sentry_types::protocol::v7::{Breadcrumb, Level, User};
