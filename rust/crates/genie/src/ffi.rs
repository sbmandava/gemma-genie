//! In-process inference via litert-lm's C API (`c/engine.h`), linking the
//! prebuilt liblitert-lm.so. Optional: enabled with `--features ffi`. M6.
//!
//! Uses the conversation API (the path the litert-lm CLI uses), which renders
//! the chat template internally and exchanges JSON messages — unlike the
//! low-level session_generate_content, which yields degenerate output.
#![allow(non_camel_case_types)]

use anyhow::{bail, Result};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// Opaque handles.
pub enum LiteRtLmEngineSettings {}
pub enum LiteRtLmEngine {}
pub enum LiteRtLmConversation {}
pub enum LiteRtLmConversationConfig {}
pub enum LiteRtLmConversationOptionalArgs {}
pub enum LiteRtLmJsonResponse {}

extern "C" {
    fn litert_lm_set_min_log_level(level: c_int);
    fn litert_lm_engine_settings_create(
        model_path: *const c_char,
        backend: *const c_char,
        vision_backend: *const c_char,
        audio_backend: *const c_char,
    ) -> *mut LiteRtLmEngineSettings;
    fn litert_lm_engine_settings_delete(s: *mut LiteRtLmEngineSettings);
    fn litert_lm_engine_create(s: *const LiteRtLmEngineSettings) -> *mut LiteRtLmEngine;
    fn litert_lm_engine_delete(e: *mut LiteRtLmEngine);
    fn litert_lm_conversation_create(
        engine: *mut LiteRtLmEngine,
        config: *mut LiteRtLmConversationConfig,
    ) -> *mut LiteRtLmConversation;
    fn litert_lm_conversation_delete(conv: *mut LiteRtLmConversation);
    fn litert_lm_conversation_send_message(
        conv: *mut LiteRtLmConversation,
        message_json: *const c_char,
        extra_context: *const c_char,
        optional_args: *const LiteRtLmConversationOptionalArgs,
    ) -> *mut LiteRtLmJsonResponse;
    fn litert_lm_json_response_get_string(resp: *const LiteRtLmJsonResponse) -> *const c_char;
    fn litert_lm_json_response_delete(resp: *mut LiteRtLmJsonResponse);
}

/// One-shot in-process generation via the conversation API. `backend`="gpu"/"cpu".
pub fn generate(model_path: &str, backend: &str, prompt: &str) -> Result<String> {
    let mp = CString::new(model_path)?;
    let be = CString::new(backend)?;
    let msg = serde_json::json!({ "role": "user", "content": prompt }).to_string();
    let msg_c = CString::new(msg)?;
    let ctx_c = CString::new("{}")?;

    unsafe {
        litert_lm_set_min_log_level(4); // ERROR
        let settings = litert_lm_engine_settings_create(
            mp.as_ptr(),
            be.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
        );
        if settings.is_null() {
            bail!("litert_lm_engine_settings_create returned NULL");
        }
        let engine = litert_lm_engine_create(settings);
        if engine.is_null() {
            litert_lm_engine_settings_delete(settings);
            bail!("litert_lm_engine_create returned NULL");
        }
        let conv = litert_lm_conversation_create(engine, std::ptr::null_mut());
        if conv.is_null() {
            litert_lm_engine_delete(engine);
            litert_lm_engine_settings_delete(settings);
            bail!("litert_lm_conversation_create returned NULL");
        }
        let resp = litert_lm_conversation_send_message(
            conv,
            msg_c.as_ptr(),
            ctx_c.as_ptr(),
            std::ptr::null(),
        );
        let mut out = String::new();
        if !resp.is_null() {
            let s = litert_lm_json_response_get_string(resp);
            if !s.is_null() {
                let json = CStr::from_ptr(s).to_string_lossy().into_owned();
                out = extract_text(&json);
            }
            litert_lm_json_response_delete(resp);
        }
        litert_lm_conversation_delete(conv);
        litert_lm_engine_delete(engine);
        litert_lm_engine_settings_delete(settings);
        if out.trim().is_empty() {
            bail!("generation produced no output");
        }
        Ok(out)
    }
}

/// Pull the assistant text out of the JSON response
/// (`{"content":[{"type":"text","text":"..."}]}`, or simpler shapes).
fn extract_text(json: &str) -> String {
    let v: serde_json::Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(_) => return json.to_string(),
    };
    if let Some(arr) = v.get("content").and_then(|c| c.as_array()) {
        let s: String = arr
            .iter()
            .filter_map(|it| it.get("text").and_then(|t| t.as_str()))
            .collect();
        if !s.is_empty() {
            return s;
        }
    }
    if let Some(c) = v.get("content").and_then(|c| c.as_str()) {
        return c.to_string();
    }
    if let Some(t) = v.get("text").and_then(|t| t.as_str()) {
        return t.to_string();
    }
    json.to_string()
}
