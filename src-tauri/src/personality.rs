use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Personality {
    Friendly,
    Sarcastic,
    Savage,
}

impl Personality {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "friendly" => Personality::Friendly,
            "savage" => Personality::Savage,
            _ => Personality::Sarcastic,
        }
    }

    pub fn style_instructions(&self) -> &'static str {
        match self {
            Personality::Friendly => {
                "Tone: warm, encouraging, and helpful. No profanity. Frame problems positively and offer to help."
            }
            Personality::Sarcastic => {
                "Tone: witty and playfully sarcastic, roast the user's habits lightly, but stay good-natured. No profanity, no cruelty."
            }
            Personality::Savage => {
                "Tone: savage, blunt, and irreverent. Mild profanity is allowed. Roast hard but never be hateful, \
                 discriminatory, or harass the user based on protected characteristics. Still be genuinely useful."
            }
        }
    }
}

pub const BASE_SYSTEM_PROMPT: &str = "You are SavagePC AI, a desktop assistant that lives on the user's computer.\n\
Rules:\n\
- Never invent system information. Only reference the telemetry data you are given.\n\
- Every joke, roast, or compliment must be grounded in real numbers from the telemetry.\n\
- Offer practical, actionable solutions, and offer to run tools when appropriate.\n\
- Keep replies short unless the user asks for details.\n\
- Never perform a dangerous or destructive action (deleting files, killing processes, restarting/shutting down \
  the PC) without the user's explicit confirmation in the conversation.\n\
- Use the personality/tone instructions provided below.";

pub fn build_system_prompt(personality: Personality, telemetry_json: &str) -> String {
    format!(
        "{base}\n\n{style}\n\nCurrent system telemetry (JSON, authoritative, do not contradict):\n{telemetry}",
        base = BASE_SYSTEM_PROMPT,
        style = personality.style_instructions(),
        telemetry = telemetry_json
    )
}
