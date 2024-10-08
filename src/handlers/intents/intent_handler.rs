use std::collections::HashMap;
use crate::handlers::intents::{
    handle_informative_intent, handle_empathetic_intent, handle_task_oriented_intent, handle_others_intent,
};
use crate::handlers::intents::intents::Intent;

type IntentHandler = fn(&str) -> String;

pub fn intent_functions(intents: &[Intent]) -> HashMap<String, IntentHandler> {
    let mut map: HashMap<String, IntentHandler> = HashMap::new();
    for intent in intents {
        match intent.name.as_str() {
            "informative" => map.insert(intent.name.clone(), handle_informative_intent),
            "empathetic" => map.insert(intent.name.clone(), handle_empathetic_intent),
            "task_oriented" => map.insert(intent.name.clone(), handle_task_oriented_intent),
            "others" => map.insert(intent.name.clone(), handle_others_intent),
            _ => None,
        };
    }
    map
}

pub fn handle_intent(intent_name: &str, query: &str, intents: &[Intent]) -> String {
    let handlers = intent_functions(intents);
    
    if let Some(handler) = handlers.get(intent_name) {
        handler(query)
    } else {
        format!("Unrecognized intent: {}", intent_name)
    }
}