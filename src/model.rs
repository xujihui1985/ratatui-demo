use crate::components::{conversation::Conversation, input::Input, status::Status};

/// The application's state.
pub struct Model {
    pub conversation: Conversation,
    pub input: Input,
    pub status: Status,
    pub state: AppState,
}

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

impl Model {
    /// Creates a new model.
    pub fn new() -> Self {
        let mut model = Self {
            conversation: Conversation::new(),
            input: Input::new(),
            status: Status::new(),
            state: AppState::Idle,
        };
        model.update_components_state();
        model
    }

    fn update_components_state(&mut self) {
        self.input.state = self.state;
        self.status.state = self.state;
    }

    /// Submits the user's input.
    pub fn submit(&mut self) {
        // Add the user's message to the conversation.
        self.conversation
            .conversation
            .push(Message::User(self.input.input.clone()));

        // Clear the input box.
        self.input.input.clear();

        self.set_state(AppState::Busy);
    }

    pub fn process_response(&mut self) {
        // Simulate a model response.
        self.conversation.conversation.push(Message::Model(
            "I'm a Ratatui-based Gemini CLI replica. How can I help you?".to_string(),
        ));
        self.conversation.conversation.push(Message::ToolCode(
            "print(default_api.list_directory(path = \"_\"))".to_string(),
        ));

        self.set_state(AppState::Idle);
    }

    pub fn set_state(&mut self, state: AppState) {
        self.state = state;
        self.update_components_state();
    }
}