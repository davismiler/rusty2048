use rusty2048_shared::{I18n, Language, TranslationKey};
use std::fs;
use std::path::Path;

/// Language manager for CLI version
pub struct LanguageManager {
    i18n: I18n,
    config_file: String,
}

impl LanguageManager {
    /// Create a new language manager
    pub fn new() -> Self {
        let mut manager = Self {
            i18n: I18n::new(),
            config_file: "cli/language_config.json".to_string(),
        };
        
        // Load saved language preference
        manager.load_language_preference();
        
        manager
    }
    
    /// Get current language
    pub fn current_language(&self) -> Language {
        self.i18n.current_language()
    }
    
    /// Set language
    pub fn set_language(&mut self, language: Language) {
        self.i18n.set_language(language);
        self.save_language_preference();
    }
    
    /// Get translation
    pub fn t(&self, key: &TranslationKey) -> String {
        self.i18n.t(key)
    }
    
    /// Get translation with parameters
    #[allow(dead_code)]
    pub fn t_with_params(&self, key: &TranslationKey, params: &[(&str, &str)]) -> String {
        self.i18n.t_with_params(key, params)
    }
    
    /// Format duration
    #[allow(dead_code)]
    pub fn format_duration(&self, seconds: u64) -> String {
        self.i18n.format_duration(seconds)
    }
    
    /// Get supported languages
    pub fn supported_languages(&self) -> Vec<Language> {
        self.i18n.supported_languages()
    }
    
    /// Cycle to next language
    pub fn next_language(&mut self) {
        let languages = self.supported_languages();
        let current_index = languages.iter().position(|&l| l == self.current_language()).unwrap_or(0);
        let next_index = (current_index + 1) % languages.len();
        self.set_language(languages[next_index]);
    }
    
    /// Cycle to previous language
    #[allow(dead_code)]
    pub fn prev_language(&mut self) {
        let languages = self.supported_languages();
        let current_index = languages.iter().position(|&l| l == self.current_language()).unwrap_or(0);
        let prev_index = if current_index == 0 {
            languages.len() - 1
        } else {
            current_index - 1
        };
        self.set_language(languages[prev_index]);
    }
    
    /// Load language preference from file
    fn load_language_preference(&mut self) {
        if !Path::new(&self.config_file).exists() {
            return;
        }
        
        if let Ok(content) = fs::read_to_string(&self.config_file) {
            if let Ok(language_code) = serde_json::from_str::<String>(&content) {
                if let Some(language) = Language::from_code(&language_code) {
                    self.i18n.set_language(language);
                }
            }
        }
    }
    
    /// Save language preference to file
    fn save_language_preference(&self) {
        // Ensure directory exists
        if let Some(parent) = Path::new(&self.config_file).parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        // Save language code
        let language_code = self.current_language().code();
        if let Ok(json) = serde_json::to_string(&language_code) {
            let _ = fs::write(&self.config_file, json);
        }
    }
    
    /// Get language display name
    pub fn language_name(&self) -> &'static str {
        self.current_language().name()
    }
    
    /// Get language code
    pub fn language_code(&self) -> &'static str {
        self.current_language().code()
    }
}

impl Default for LanguageManager {
    fn default() -> Self {
        Self::new()
    }
}
