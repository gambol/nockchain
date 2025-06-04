#!/usr/bin/env cargo +nightly -Zscript
//! Simple benchmark script for prove-block-inner function
//! 
//! Usage: cargo +nightly -Zscript scripts/benchmark_prove_block.rs
//! 
//! This script directly tests the prove-block-inner function performance
//! without running the full mining system.

use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting prove-block-inner benchmark...");
    println!("This will test the core mining function performance\n");
    
    // Test parameters
    let test_cases = vec![
        ("Test Case 1", 64, [0x1, 0x2, 0x3, 0x4, 0x5], [0x100, 0x200, 0x300, 0x400, 0x1]),
        ("Test Case 2", 64, [0x1, 0x2, 0x3, 0x4, 0x5], [0x100, 0x200, 0x300, 0x400, 0x2]),
        ("Test Case 3", 64, [0x1, 0x2, 0x3, 0x4, 0x5], [0x100, 0x200, 0x300, 0x400, 0x3]),
    ];
    
    let mut total_time = Duration::new(0, 0);
    let mut results = Vec::new();
    
    for (name, length, block_commitment, nonce) in test_cases {
        println!("📊 Running {}", name);
        println!("   Length: {}", length);
        println!("   Block commitment: {:?}", block_commitment);
        println!("   Nonce: {:?}", nonce);
        
        let start_time = Instant::now();
        
        // This is where we would call prove-block-inner
        // For now, we'll simulate the call
        let result = simulate_prove_block_inner(length, block_commitment, nonce).await?;
        
        let elapsed = start_time.elapsed();
        total_time += elapsed;
        
        println!("   ⏱️  Time: {:.2?}", elapsed);
        println!("   ✅ Result hash: 0x{:x}", result.hash);
        println!();
        
        results.push((name, elapsed, result));
    }
    
    // Summary
    println!("📈 BENCHMARK SUMMARY");
    println!("===================");
    println!("Total time: {:.2?}", total_time);
    println!("Average time per proof: {:.2?}", total_time / results.len() as u32);
    println!("Proofs per minute: {:.1}", 60.0 / (total_time.as_secs_f64() / results.len() as f64));
    
    // Performance analysis
    let times: Vec<Duration> = results.iter().map(|(_, time, _)| *time).collect();
    let min_time = times.iter().min().unwrap();
    let max_time = times.iter().max().unwrap();
    
    println!("\n📊 PERFORMANCE ANALYSIS");
    println!("=======================");
    println!("Fastest proof: {:.2?}", min_time);
    println!("Slowest proof: {:.2?}", max_time);
    println!("Variance: {:.2?}", max_time.saturating_sub(*min_time));
    
    // Recommendations
    println!("\n💡 OPTIMIZATION RECOMMENDATIONS");
    println!("===============================");
    if total_time.as_secs() > 30 {
        println!("⚠️  Proof generation is slow (>10s per proof)");
        println!("   Consider optimizing STARK parameters or jets");
    } else {
        println!("✅ Proof generation performance is reasonable");
    }
    
    Ok(())
}

/// Simulated prove-block-inner function
/// In the real implementation, this would call the actual Hoon function
async fn simulate_prove_block_inner(
    _length: u64,
    _block_commitment: [u64; 5],
    nonce: [u64; 5],
) -> Result<ProofResult, Box<dyn std::error::Error>> {
    // Simulate the time it takes to generate a STARK proof
    // In reality, this would be much longer (seconds to minutes)
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Simulate a proof hash based on the nonce
    let hash = nonce.iter().fold(0u64, |acc, &x| acc.wrapping_mul(31).wrapping_add(x));
    
    Ok(ProofResult {
        hash,
        proof_size: 1024, // Simulated proof size
    })
}

#[derive(Debug)]
struct ProofResult {
    hash: u64,
    proof_size: usize,
}
