extern crate test_gates;

// This test will run only if TEST_ENV_VAR1 is set
#[test]
#[test_gates::require_env("TEST_ENV_VAR1")]
fn test_single_env_var() {
    // This part will only execute if TEST_ENV_VAR1 is set
    let var = std::env::var("TEST_ENV_VAR1").unwrap();
    println!("TEST_ENV_VAR1 = {}", var);
    assert!(true);
}

// This test requires multiple env vars to run
#[test]
#[test_gates::require_env("TEST_ENV_VAR1", "TEST_ENV_VAR2")]
fn test_multiple_env_vars() {
    // This part will only execute if both TEST_ENV_VAR1 and TEST_ENV_VAR2 are set
    let var1 = std::env::var("TEST_ENV_VAR1").unwrap();
    let var2 = std::env::var("TEST_ENV_VAR2").unwrap();
    
    println!("TEST_ENV_VAR1 = {}", var1);
    println!("TEST_ENV_VAR2 = {}", var2);
    
    assert!(true);
}

// This test will always run (no env vars required)
#[test]
fn test_always_runs() {
    // This test doesn't require any env vars
    println!("This test always runs because it doesn't require any env vars");
    assert!(true);
}

// Example showing how it would work with tokio
#[test]
#[test_gates::require_env("TEST_ENV_VAR1")]
// In a real project with tokio, you would use:
// #[tokio::test]
fn test_with_env_var() {
    // This test requires TEST_ENV_VAR1
    println!("Test with environment variable is running");
    assert!(true);
} 