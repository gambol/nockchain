use std::env;

// Import the verifier from our test module
// Note: In a real implementation, this would be a proper library module
use nockchain::tests::stark_proof_verifier::verify_proof_file;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 STARK Proof Verification Tool");
    println!("================================");
    println!("");

    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: {} <proof_file.json>", args[0]);
        println!("");
        println!("Example:");
        println!("  {} benchmark_results/minimal_extraction_test_20250608_055801.json", args[0]);
        return Ok(());
    }

    let proof_file = &args[1];
    
    // Check if file exists
    if !std::path::Path::new(proof_file).exists() {
        println!("❌ Error: File not found: {}", proof_file);
        return Ok(());
    }

    println!("📁 Input file: {}", proof_file);
    println!("");

    // Verify the proof
    match verify_proof_file(proof_file).await {
        Ok(()) => {
            println!("");
            println!("✅ Verification completed successfully!");
            println!("📁 Check the verification_results/ directory for detailed results.");
        }
        Err(e) => {
            println!("");
            println!("❌ Verification failed: {}", e);
        }
    }

    Ok(())
}
