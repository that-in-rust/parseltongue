/**
 * Results Component Pyramid:
 * L1: Core results display
 * L2: Performance comparison
 * L3: Animation effects
 * L4: Interactive elements
 */

import { motion } from 'framer-motion';
import { AnalysisResult } from '../types/common';
import { CONFIG } from '../config/constants';

interface ResultsCardProps {
  javaResults?: AnalysisResult;
  rustResults?: AnalysisResult;
}

export function ResultsCard({ javaResults, rustResults }: ResultsCardProps) {
  const hasComparison = javaResults && rustResults;
  const speedDiff = hasComparison 
    ? ((javaResults.processingTimeMs - rustResults.processingTimeMs) / javaResults.processingTimeMs * 100).toFixed(1)
    : null;

  return (
    <motion.div 
      className="bg-card p-6 rounded-lg mt-4"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
    >
      <h2 className="text-lg font-medium mb-4">Performance Battle</h2>
      
      <div className="grid grid-cols-2 gap-8">
        {/* Java Results */}
        <div>
          <div className="flex items-center gap-2 mb-2">
            <span style={{ color: CONFIG.BACKENDS.java.color }}>☕</span>
            <span className="font-medium">Java</span>
          </div>
          {javaResults && (
            <motion.div 
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="font-mono text-dim"
            >
              <div>{(javaResults.processingTimeMs / 1000).toFixed(1)}s</div>
              <div>{javaResults.totalFiles} files</div>
              <div>{javaResults.filesPerSecond.toFixed(0)} files/sec</div>
            </motion.div>
          )}
        </div>

        {/* Rust Results */}
        <div>
          <div className="flex items-center gap-2 mb-2">
            <span style={{ color: CONFIG.BACKENDS.rust.color }}>🦀</span>
            <span className="font-medium">Rust</span>
          </div>
          {rustResults && (
            <motion.div 
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="font-mono text-dim"
            >
              <div>{(rustResults.processingTimeMs / 1000).toFixed(1)}s</div>
              <div>{rustResults.totalFiles} files</div>
              <div>{rustResults.filesPerSecond.toFixed(0)} files/sec</div>
            </motion.div>
          )}
        </div>
      </div>

      {speedDiff && (
        <motion.div 
          className="mt-4 text-center font-medium"
          initial={{ scale: 0.9, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          transition={{ delay: 0.3 }}
        >
          <span className="text-accent">
            🏆 {Math.abs(parseFloat(speedDiff))}% {parseFloat(speedDiff) > 0 ? 'faster' : 'slower'}!
          </span>
        </motion.div>
      )}
    </motion.div>
  );
} 