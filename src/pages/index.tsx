/**
 * Page Component Pyramid:
 * L1: Core layout and state
 * L2: Analysis workflow
 * L3: Progress tracking
 * L4: Results visualization
 */

import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { UploadCard } from '../components/UploadCard';
import { ProgressCard } from '../components/ProgressCard';
import { ResultsCard } from '../components/ResultsCard';
import { ErrorState } from '../components/ErrorState';
import { AnalysisController } from '../lib/analysis';
import { Backend } from '../types/common';
import { CONFIG } from '../config/constants';

export default function Home() {
  const [controller, setController] = useState<AnalysisController>();
  const [jobId, setJobId] = useState<string>();
  const [progress, setProgress] = useState(0);
  const [currentFile, setCurrentFile] = useState<string>();
  const [results, setResults] = useState<any>();
  const [error, setError] = useState<Error>();
  const [backend, setBackend] = useState<Backend>('java');

  useEffect(() => {
    let pollInterval: NodeJS.Timeout;

    const pollProgress = async () => {
      if (!controller || !jobId) return;

      try {
        const status = await controller.getProgress();
        setProgress(status.progress);
        setCurrentFile(status.currentFile);

        if (status.status === 'complete') {
          const results = await controller.getResults();
          setResults(results);
          clearInterval(pollInterval);
        }
      } catch (e) {
        setError(e as Error);
        clearInterval(pollInterval);
      }
    };

    if (jobId) {
      pollInterval = setInterval(pollProgress, 1000);
    }

    return () => clearInterval(pollInterval);
  }, [controller, jobId]);

  const startAnalysis = async (selectedBackend: Backend) => {
    try {
      setError(undefined);
      setBackend(selectedBackend);
      setProgress(0);
      setResults(undefined);

      const newController = new AnalysisController(selectedBackend);
      setController(newController);

      const newJobId = await newController.startAnalysis();
      setJobId(newJobId);
    } catch (e) {
      setError(e as Error);
    }
  };

  return (
    <div className="min-h-screen bg-surface text-text">
      <main className="container mx-auto px-4 py-8 max-w-2xl">
        <motion.h1 
          className="text-3xl font-medium mb-8 flex items-center gap-2"
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
        >
          🐍 Parseltongue
        </motion.h1>

        <AnimatePresence mode="wait">
          <UploadCard 
            onUpload={startAnalysis}
            isProcessing={!!jobId && !results && !error}
          />

          {error && (
            <ErrorState 
              error={error}
              retry={() => startAnalysis(backend)}
            />
          )}

          {jobId && !error && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
            >
              <ProgressCard 
                currentBackend={backend}
                progress={progress}
                currentFile={currentFile}
                speed={results?.filesPerSecond}
              />

              {results && (
                <ResultsCard
                  javaResults={results.java}
                  rustResults={results.rust}
                />
              )}
            </motion.div>
          )}
        </AnimatePresence>
      </main>
    </div>
  );
} 