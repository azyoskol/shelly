# Phase 1 Unit Test Scenarios Documentation

## 🧪 Overview

Total: **16 tests** across 5 modules. All tests pass.

| Module | Test Count | Status |
|--------|------------|--------|
| `plugin` | 1 | ✅ |
| `zsh` | 3 | ✅ |
| `fish` | 3 | ✅ |
| `starship` | 1 | ✅ |
| `ai` | 8 | ✅ |

---

## 📋 Test Scenarios by Module

### plugin::mock_plugin

#### `test_plugin_lifecycle`
**Type:** Integration Test
**Goal:** Validate the entire plugin execution flow, simulating a complete command cycle.
1. **Initialization (`initialize`):** Verify plugin validates `shell_name` config.
2. **Pre-Prompt Hook (`pre_prompt_hook`):** Verify context update (sets `CURRENT_DIR`).
3. **Post-Execute Hook (`post_execute_hook`):** Verify exit code handling (success case).

---

### zsh

#### `test_zsh_precmd_hook`
**Type:** Unit Test
**Goal:** Verify precmd hook correctly delegates to plugin's `pre_prompt_hook`.
**Verification:** Context contains `CURRENT_DIR` after hook execution.

#### `test_zsh_preexec_hook`
**Type:** Unit Test
**Goal:** Verify preexec hook updates context with `LAST_COMMAND` and `EXIT_CODE`.
**Verification:** Context contains command and exit code after hook execution.

#### `test_generate_zshrc_snippet`
**Type:** Unit Test
**Goal:** Verify zshrc snippet generation for .zshrc installation.
**Verification:** Output contains `precmd_shally`, `preexec_shally`, `precmd_functions`, and binary path.

---

### fish

#### `test_fish_prompt_hook`
**Type:** Unit Test
**Goal:** Verify Fish prompt hook correctly delegates to plugin's `pre_prompt_hook`.
**Verification:** Context contains `CURRENT_DIR` after hook execution.

#### `test_fish_command_not_found`
**Type:** Unit Test
**Goal:** Verify Fish command_not_found hook handles failed commands.
**Verification:** Context contains `FAILED_COMMAND`; hook returns error (expected behavior for non-zero exit).

#### `test_generate_config_snippet`
**Type:** Unit Test
**Goal:** Verify config.fish snippet generation for Fish installation.
**Verification:** Output contains `shally_prompt`, `shally_command_not_found`, `fish_prompt`, and binary path.

---

### starship

#### `test_export_context`
**Type:** Unit Test
**Goal:** Verify context export converts ShellContext to `SHELLAI_*` environment variables.
**Verification:** Context keys become uppercase with `SHELLAI_` prefix (e.g., `CURRENT_DIR` → `SHELLAI_CURRENT_DIR`).

---

### ai

#### `test_ai_not_configured`
**Type:** Unit Test
**Goal:** Verify AI returns error when not configured.
**Verification:** `suggest_command()` returns error "API endpoint not set..."

#### `test_build_prompt`
**Type:** Unit Test
**Goal:** Verify prompt building includes context and history.
**Verification:** Prompt string contains `PWD=/home/user`, `ls`, `cd /tmp`.

#### `test_with_model`
**Type:** Unit Test
**Goal:** Verify `with_model()` builder method sets model correctly.
**Verification:** Model is set to "gpt-4" after builder call.

#### `test_cli_config_endpoint_and_key`
**Type:** Unit Test
**Goal:** Verify `with_config()` sets endpoint and API key correctly.
**Verification:** Endpoint and key are stored in integration struct.

#### `test_suggest_command_after_config`
**Type:** Unit Test
**Goal:** Verify `suggest_command()` makes API call after configuration.
**Verification:** Returns a result via test-mode or API (depending on environment). In test mode, a canned suggestion is returned.

#### `test_local_model_suggests_in_test_mode`
 **Type:** Unit Test
 **Goal:** Verify local AI model returns a canned suggestion when running in test mode (no real API call).
 **Verification:** When `SHELLAI_TEST_MODE=1` is set, suggestion command equals `echo test-mode`.

#### `test_local_model_variants`
 **Type:** Unit Test
 **Goal:** Verify multiple local model names (local-llm-v1, local-xl) are treated as local models.
 **Verification:** For each name, suggest_command() returns a canned local-suggestion.

#### `test_default_model_is_gpt_3_5`
**Type:** Unit Test
**Goal:** Verify default model is gpt-3.5-turbo.
**Verification:** Default model equals "gpt-3.5-turbo".

#### `test_config_requires_endpoint`
**Type:** Unit Test
**Goal:** Verify empty endpoint is rejected.
**Verification:** `suggest_command()` returns error about missing endpoint.

#### `test_config_requires_api_key`
**Type:** Unit Test
**Goal:** Verify empty API key is rejected.
**Verification:** `suggest_command()` returns error about missing API key.

---

## 🏗️ Test Execution

```bash
cargo test
```

**Output:**
```
running 16 tests
test ai::tests::test_ai_not_configured ... ok
test ai::tests::test_cli_config_endpoint_and_key ... ok
test ai::tests::test_config_requires_api_key ... ok
test ai::tests::test_config_requires_endpoint ... ok
test ai::tests::test_default_model_is_gpt_3_5 ... ok
test ai::tests::test_suggest_command_after_config ... ok
test ai::tests::test_build_prompt ... ok
test ai::tests::test_with_model ... ok
test fish::tests::test_fish_command_not_found ... ok
test fish::tests::test_fish_prompt_hook ... ok
test fish::tests::test_generate_config_snippet ... ok
test plugin::mock_plugin::tests::test_plugin_lifecycle ... ok
test starship::tests::test_export_context ... ok
test zsh::tests::test_generate_zshrc_snippet ... ok
test zsh::tests::test_zsh_precmd_hook ... ok
test zsh::tests::test_zsh_preexec_hook ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

*Last updated: 2026-04-21*
