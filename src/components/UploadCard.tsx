/**
 * Upload Component Pyramid:
 * L1: Core file input
 * L2: Backend selector
 * L3: Upload state
 * L4: Error handling
 */

import { useState } from 'react';
import { motion } from 'framer-motion';
import { Backend } from '../types/common';

interface UploadCardProps {
  onUpload: (backend: Backend) => Promise<void>;
  isProcessing: boolean;
}

export function UploadCard({ onUpload, isProcessing }: UploadCardProps) {
  const [selectedBackend, setSelectedBackend] = useState<Backend>('java');

  return (
    <div className="bg-card p-6 rounded-lg">
      <div className="flex gap-4 mb-4">
        <label className="flex items-center">
          <input
            type="radio"
            value="java"
            checked={selectedBackend === 'java'}
            onChange={(e) => setSelectedBackend(e.target.value as Backend)}
            className="mr-2"
          />
          <span className="text-java">☕ Java</span>
        </label>
        <label className="flex items-center">
          <input
            type="radio"
            value="rust"
            checked={selectedBackend === 'rust'}
            onChange={(e) => setSelectedBackend(e.target.value as Backend)}
            className="mr-2"
          />
          <span className="text-rust">🦀 Rust</span>
        </label>
      </div>

      <motion.button
        onClick={() => onUpload(selectedBackend)}
        className="bg-accent hover:bg-accent/90 text-white px-6 py-3 rounded-lg w-full"
        whileHover={{ scale: 1.02 }}
        whileTap={{ scale: 0.98 }}
        disabled={isProcessing}
      >
        ⚡ Start Analysis
      </motion.button>
    </div>
  );
} 