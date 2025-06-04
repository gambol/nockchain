#!/bin/bash

# Monitor script for long-running prove-block-inner benchmarks
# This helps track progress during the long STARK proof generation

echo "🔍 Nockchain Benchmark Monitor"
echo "=============================="
echo "This script monitors system resources during benchmark execution"
echo ""

# Function to get process info
get_benchmark_processes() {
    ps aux | grep -E "(cargo|prove_block|nockchain)" | grep -v grep | grep -v monitor
}

# Function to display system stats
show_system_stats() {
    echo "📊 System Statistics ($(date))"
    echo "----------------------------------------"
    
    # CPU usage
    if command -v top &> /dev/null; then
        echo "🖥️  CPU Usage:"
        top -l 1 -n 0 | grep "CPU usage" || echo "   CPU info not available"
    fi
    
    # Memory usage
    if command -v free &> /dev/null; then
        echo "💾 Memory Usage:"
        free -h
    elif command -v vm_stat &> /dev/null; then
        echo "💾 Memory Usage (macOS):"
        vm_stat | head -5
    fi
    
    # Disk usage for temp directories
    echo "💿 Temp Directory Usage:"
    df -h /tmp 2>/dev/null || echo "   Temp directory info not available"
    
    echo ""
}

# Function to show benchmark processes
show_benchmark_processes() {
    local processes=$(get_benchmark_processes)
    
    if [ -n "$processes" ]; then
        echo "🔄 Active Benchmark Processes:"
        echo "------------------------------"
        echo "$processes" | while read line; do
            echo "   $line"
        done
        echo ""
        
        # Show process tree if available
        if command -v pstree &> /dev/null; then
            echo "🌳 Process Tree:"
            pstree -p $(pgrep -f "prove_block" | head -1) 2>/dev/null || echo "   Process tree not available"
            echo ""
        fi
    else
        echo "❌ No benchmark processes found"
        echo "   The benchmark may have completed or not started yet"
        echo ""
    fi
}

# Function to estimate remaining time
estimate_progress() {
    echo "⏱️  Time Estimation:"
    echo "-------------------"
    echo "   Single STARK proof: 5-15 minutes"
    echo "   Multiple proofs (3x): 15-45 minutes"
    echo "   Full benchmark suite: 30-60 minutes"
    echo ""
    echo "💡 Progress indicators to look for:"
    echo "   - 'Setting up kernel...' - Initial setup (1-2 minutes)"
    echo "   - 'Starting STARK proof generation...' - Main computation begins"
    echo "   - High CPU usage (90-100%) - Proof generation in progress"
    echo "   - 'STARK proof completed!' - Individual proof finished"
    echo ""
}

# Main monitoring loop
monitor_continuously() {
    local interval=${1:-30}  # Default 30 seconds
    
    echo "🔄 Starting continuous monitoring (every ${interval}s)"
    echo "Press Ctrl+C to stop monitoring"
    echo ""
    
    while true; do
        clear
        echo "🔍 Nockchain Benchmark Monitor - $(date)"
        echo "========================================"
        echo ""
        
        show_benchmark_processes
        show_system_stats
        estimate_progress
        
        echo "⏰ Next update in ${interval} seconds..."
        sleep "$interval"
    done
}

# Parse command line arguments
case "${1:-status}" in
    "continuous"|"monitor"|"watch")
        interval=${2:-30}
        monitor_continuously "$interval"
        ;;
    
    "status"|"check")
        show_benchmark_processes
        show_system_stats
        ;;
    
    "help"|"-h"|"--help")
        echo "Usage: $0 [command] [interval]"
        echo ""
        echo "Commands:"
        echo "  status, check       - Show current status (default)"
        echo "  continuous, monitor - Continuously monitor (default: 30s interval)"
        echo "  help               - Show this help"
        echo ""
        echo "Examples:"
        echo "  $0                 # Show current status"
        echo "  $0 monitor         # Monitor continuously (30s interval)"
        echo "  $0 monitor 60      # Monitor continuously (60s interval)"
        echo ""
        echo "💡 Tips:"
        echo "  - Run this in a separate terminal while benchmark is running"
        echo "  - Look for high CPU usage to confirm proof generation is active"
        echo "  - Monitor memory usage to ensure system isn't swapping"
        ;;
    
    *)
        echo "❌ Unknown command: $1"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac
