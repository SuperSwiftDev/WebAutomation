// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — SETTINGS
// ————————————————————————————————————————————————————————————————————————————

use std::ops::Not;

use crate::ast::{document::{DocumentChildCode, DocumentNode}, prompt::{PromptChildNode, PromptNode}};
use crate::common::{message::MessageRole, prompt::{PromptSettings, ResponseFormatType}};

#[derive(Debug, Clone)]
pub struct RuntimeEnvironment {
    pub api_key: String,
}

#[derive(Debug, Clone)]
pub struct DocumentInvocation {
    pub runtime_environment: RuntimeEnvironment,
    pub target_prompt: String,
}

// ————————————————————————————————————————————————————————————————————————————
// DATA MODEL — CONVERSATION
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct ConversationMessage {
    pub message: ai_client::request::Message,
    pub evaluated: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Conversation {
    pub messages: Vec<ConversationMessage>,
    pub prompt_settings: PromptSettings,
}

impl Conversation {
    pub fn already_evaluated(&self) -> bool {
        self.messages.last().map(|x| x.evaluated).unwrap_or(false)
    }
}


#[derive(Debug, Clone)]
pub struct PromptContext {
    pub runtime_environment: RuntimeEnvironment,
    pub conversation: Conversation,
}

impl PromptContext {
    pub fn new(runtime_environment: RuntimeEnvironment) -> Self {
        Self {
            runtime_environment,
            conversation: Default::default(),
        }
    }
    pub async fn invoke(&mut self) -> String {
        let messages = self.conversation.messages
            .iter()
            .map(|x| x.message.clone())
            .collect::<Vec<_>>();
        invoke(&messages, &self.runtime_environment, &self.conversation.prompt_settings).await
    }
    pub fn to_snapshot(&self) -> crate::snapshot::ConversationSnapshot {
        let messages = self.conversation.messages
            .iter()
            .map(|x| {
                crate::snapshot::MessageSnapshot {
                    message_payload: x.message.clone(),
                    evaluation_point: x.evaluated,
                }
            })
            .collect::<Vec<_>>();
        crate::snapshot::ConversationSnapshot {
            messages,
        }
    }
}


// ————————————————————————————————————————————————————————————————————————————
// REQUEST HANDLER
// ————————————————————————————————————————————————————————————————————————————

async fn invoke(
    messages: &[ai_client::request::Message],
    runtime_environment: &RuntimeEnvironment,
    prompt_settings: &PromptSettings,
) -> String {
    use ai_client::client::URL;
    use ai_client::request::OpenAiModels;
    let request_builder = prompt_settings.request_builder()
        .with_messages(messages.to_owned())
        .with_model(OpenAiModels::gpt_4)
        .with_stream(true);
    let client_builder = ai_client::client::ClientBuilder::default()
        .with_api_url(URL::OPEN_AI_CHAT_COMPLETIONS)
        .with_api_key(&runtime_environment.api_key)
        .with_request_body(request_builder)
        .with_logger(ai_client::log::StdErrLogger::default().with_colorize(true));
    let client = client_builder.build_streaming_api_call().unwrap();
    let output_result = client.execute_async().await;
    let output = output_result.unwrap();
    output.content(0).unwrap()
}

impl PromptSettings {
    pub fn request_builder(&self) -> ai_client::request::RequestBuilder {
        let mut builder = ai_client::request::RequestBuilder::default();
        if let Some(model) = self.model.as_ref() {
            builder = builder.with_model(model.0.clone());
        }
        if let Some(temperature) = self.temperature.as_ref() {
            builder = builder.with_temperature(temperature.0.clone());
        }
        if let Some(n) = self.n.as_ref() {
            builder = builder.with_n(n.0.clone());
        }
        if let Some(max_tokens) = self.max_tokens.as_ref() {
            builder = builder.with_max_tokens(max_tokens.0.clone());
        }
        if let Some(top_p) = self.top_p.as_ref() {
            builder = builder.with_top_p(top_p.0.clone());
        }
        if let Some(frequency_penalty) = self.frequency_penalty.as_ref() {
            builder = builder.with_frequency_penalty(frequency_penalty.0.clone());
        }
        if let Some(presence_penalty) = self.presence_penalty.as_ref() {
            builder = builder.with_presence_penalty(presence_penalty.0.clone());
        }
        if let Some(logprobs) = self.logprobs.as_ref() {
            builder = builder.with_logprobs(logprobs.0.clone());
        }
        if let Some(top_logprobs) = self.top_logprobs.as_ref() {
            builder = builder.with_top_logprobs(top_logprobs.0.clone());
        }
        if let Some(response_format) = self.response_format.as_ref() {
            builder = match response_format.0 {
                ResponseFormatType::Text => builder.with_response_format(ai_client::request::ResponseFormat::TEXT),
                ResponseFormatType::JsonObject => builder.with_response_format(ai_client::request::ResponseFormat::JSON_OBJECT),
            };
        }
        builder
    }
}


// ————————————————————————————————————————————————————————————————————————————
// INVOCATION
// ————————————————————————————————————————————————————————————————————————————

impl DocumentNode {
    pub async fn invoke(&self, document_invocation: &DocumentInvocation) -> Result<PromptContext, ()> {
        for child in self.children.iter() {
            match child {
                DocumentChildCode::Prompt(prompt) if prompt.name() == &document_invocation.target_prompt => {
                    let prompt_context = prompt.invoke(&document_invocation.runtime_environment).await;
                    return Ok(prompt_context)
                }
                DocumentChildCode::Prompt(_) => (),
            }
        }
        Err(())
    }
}

impl PromptNode {
    pub async fn invoke(&self, runtime_environment: &RuntimeEnvironment) -> PromptContext {
        let mut prompt_context = PromptContext::new(runtime_environment.clone());
        for child in self.children.iter() {
            match child {
                PromptChildNode::Msg(msg) => {
                    let message = match msg.role {
                        MessageRole::System => {
                            ai_client::request::Message::system(msg.text_content())
                        }
                        MessageRole::User => {
                            ai_client::request::Message::user(msg.text_content())
                        }
                        MessageRole::Assistant => {
                            ai_client::request::Message::assistant(msg.text_content())
                        }
                    };
                    let message = ConversationMessage {
                        message,
                        evaluated: false,
                    };
                    prompt_context.conversation.messages.push(message);
                }
                PromptChildNode::Breakpoint(breakpoint) => {
                    let output = prompt_context.invoke().await;
                    let message = match breakpoint.role {
                        MessageRole::System => {
                            ai_client::request::Message::system(output)
                        }
                        MessageRole::User => {
                            ai_client::request::Message::user(output)
                        }
                        MessageRole::Assistant => {
                            ai_client::request::Message::assistant(output)
                        }
                    };
                    let message = ConversationMessage {
                        message,
                        evaluated: true,
                    };
                    prompt_context.conversation.messages.push(message);
                }
                PromptChildNode::Set(set) => {
                    prompt_context.conversation.prompt_settings.merge_mut(set.prompt_settings.clone());
                }
            }
        }
        if prompt_context.conversation.already_evaluated().not() {
            let output = prompt_context.invoke().await;
            let message = ai_client::request::Message::assistant(output);
            let message = ConversationMessage {
                message,
                evaluated: true,
            };
            prompt_context.conversation.messages.push(message);
        }
        prompt_context
    }
}

