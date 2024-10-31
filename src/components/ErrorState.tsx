/**
 * Error Component Pyramid:
 * L1: Core error display
 * L2: Error categorization
 * L3: Retry mechanism
 * L4: Error tracking
 */

import { motion } from 'framer-motion';
import { CONFIG } from '../config/constants';

interface ErrorStateProps {
  error: Error;
  retry: () => void;
}

export function ErrorState({ error, retry }: ErrorStateProps) {
  const errorType = error.name === 'NetworkError' ? 'network' : 'analysis';

  return (
    <motion.div
      className="bg-card p-6 rounded-lg mt-4 border border-red-500/20"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
    >
      <div className="flex items-center gap-2 mb-4">
        <span className="text-red-500">⚠️</span>
        <span className="font-medium">Analysis Failed</span>
      </div>

      <div className="font-mono text-sm text-dim mb-4">
        $ {error.message}
      </div>

      <motion.button
        onClick={retry}
        className="bg-red-500/20 hover:bg-red-500/30 text-red-500 px-4 py-2 rounded-lg text-sm"
        whileHover={{ scale: 1.02 }}
        whileTap={{ scale: 0.98 }}
      >
        ↻ Retry Analysis
      </motion.button>
    </motion.div>
  );
} 