/**
 * Progress Component Pyramid:
 * L1: Core progress display
 * L2: Backend-specific styling
 * L3: Animation states
 * L4: Performance metrics
 */

import { motion } from 'framer-motion';
import { Backend } from '../types/common';
import { CONFIG } from '../config/constants';

interface ProgressCardProps {
  currentBackend: Backend;
  progress: number;
  currentFile?: string;
  speed?: number;
}

export function ProgressCard({ currentBackend, progress, currentFile, speed }: ProgressCardProps) {
  const backendConfig = CONFIG.BACKENDS[currentBackend];

  return (
    <motion.div 
      className="bg-card p-6 rounded-lg mt-4"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
    >
      <div className="font-mono text-dim mb-4">
        $ analyzing codebase...
      </div>

      <div className="space-y-2">
        <div className="flex items-center gap-2">
          <span style={{ color: backendConfig.color }}>
            {currentBackend === 'java' ? '☕' : '🦀'}
          </span>
          <span className="font-medium">
            {currentBackend.toUpperCase()} Backend
          </span>
        </div>

        <div className="h-2 bg-surface rounded overflow-hidden">
          <motion.div
            className="h-full rounded"
            style={{ 
              background: backendConfig.gradient,
              width: `${progress}%`
            }}
            initial={{ width: 0 }}
            animate={{ width: `${progress}%` }}
            transition={{ duration: 0.3 }}
          />
        </div>

        {currentFile && (
          <div className="font-mono text-sm text-dim">
            → {currentFile}
          </div>
        )}

        {speed && (
          <div className="font-mono text-sm text-dim">
            @ {speed.toFixed(0)} files/sec
          </div>
        )}
      </div>
    </motion.div>
  );
} 