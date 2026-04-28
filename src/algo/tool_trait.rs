#![allow(dead_code)]

use super::registry::AlgorithmCategory;

/// Unified interface for all cryptographic tool states.
///
/// Concrete tools keep algorithm-specific operations on their own types; this
/// trait captures the common execution, reset, output, and error contract.
pub trait CryptoTool {
    /// Human-readable name for this tool.
    fn name(&self) -> &str;

    /// Algorithm category from the registry.
    fn category(&self) -> AlgorithmCategory;

    /// Execute the primary operation for the tool.
    fn execute(&mut self);

    /// Reset all state to defaults.
    fn reset(&mut self);

    /// Whether the tool has produced any output.
    fn has_output(&self) -> bool;

    /// Get the primary output as a display string.
    fn output_display(&self) -> String;

    /// Get error message if any.
    fn error_display(&self) -> Option<&str>;

    /// Whether the tool currently has an error.
    fn has_error(&self) -> bool {
        self.error_display().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StubTool {
        output: String,
        error: Option<String>,
    }

    impl crate::algo::tool_trait::CryptoTool for StubTool {
        fn name(&self) -> &str { "测试工具" }
        fn category(&self) -> AlgorithmCategory { AlgorithmCategory::Hash }
        fn execute(&mut self) { self.output = "ok".to_string(); }
        fn reset(&mut self) { self.output.clear(); self.error = None; }
        fn has_output(&self) -> bool { !self.output.is_empty() }
        fn output_display(&self) -> String { self.output.clone() }
        fn error_display(&self) -> Option<&str> { self.error.as_deref() }
    }

    #[test]
    fn crypto_tool_contract_exposes_common_state() {
        let mut tool = StubTool { output: String::new(), error: None };
        assert_eq!(tool.name(), "测试工具");
        assert_eq!(tool.category(), AlgorithmCategory::Hash);
        assert!(!tool.has_output());
        tool.execute();
        assert!(tool.has_output());
        assert_eq!(tool.output_display(), "ok");
        assert!(!tool.has_error());
        tool.error = Some("boom".to_string());
        assert_eq!(tool.error_display(), Some("boom"));
        assert!(tool.has_error());
        tool.reset();
        assert!(!tool.has_output());
        assert!(!tool.has_error());
    }
}
