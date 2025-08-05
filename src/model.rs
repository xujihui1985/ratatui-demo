/// The application's state.
#[derive(PartialEq, Clone, Copy)]
pub enum AppState {
    /// The user is typing in the input box.
    Idle,
    /// The application is busy processing a command.
    Busy,
    /// The application is quitting.
    Quitting,
}

/// A message in the conversation.
#[derive(Clone)]
pub enum Message {
    /// A message from the user.
    User(String),
    /// A message from the model.
    Model(String),
    /// A tool code from the model.
    ToolCode(String),
}