/// All communication from the CrMain thread to BYOND's main thread goes through this interface
/// Note: Communication in the other direction goes through cef_post_task.

use std::sync::mpsc;

pub use mpsc::channel;
pub type Sender = mpsc::Sender<Event>;
pub type Receiver = mpsc::Receiver<Event>;

pub enum Event {
    /// Called asynchronously after `CefBrowserHost::create_browser` has created the browser object
    /// Actions that should have been taken on the browser up to this point should be queued and later executed here
    BrowserCreated(cef::CefBrowser),

    /// Called when this page tries to perform a request to a special protocol such as `byond://`
    /// The request has already been cancelled at this point, but BYOND needs to know to parse and execute it
    UrlNavigate(String),
}
