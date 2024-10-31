/**
 * Analysis Service Pyramid:
 * L1: Core API client
 * L2: Progress tracking
 * L3: Error handling
 * L4: Performance monitoring
 */

import { Backend, AnalysisProgress, AnalysisResult, ErrorResponse } from '../types/common';
import { CONFIG } from '../config/constants';
import { MonitoringService } from './monitoring';

export class AnalysisController {
    private monitoring = MonitoringService.getInstance();
    private pollInterval?: NodeJS.Timeout;

    constructor(private backend: Backend) {}

    async startAnalysis(): Promise<string> {
        const response = await fetch(`${CONFIG.BACKENDS[this.backend].url}/api/analyze`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' }
        });

        if (!response.ok) {
            const error = await response.json() as ErrorResponse;
            this.monitoring.trackError(error);
            throw new Error(error.message);
        }

        const { jobId } = await response.json();
        return jobId;
    }

    async pollProgress(jobId: string, onProgress: (progress: AnalysisProgress) => void): Promise<void> {
        this.pollInterval = setInterval(async () => {
            try {
                const response = await fetch(
                    `${CONFIG.BACKENDS[this.backend].url}/api/status/${jobId}`
                );
                
                if (!response.ok) {
                    throw new Error('Failed to fetch progress');
                }

                const progress = await response.json();
                onProgress(progress);

                if (progress.status === 'complete' || progress.status === 'error') {
                    this.stopPolling();
                }
            } catch (error) {
                this.stopPolling();
                throw error;
            }
        }, CONFIG.POLL_INTERVAL);
    }

    async getResults(jobId: string): Promise<AnalysisResult> {
        const response = await fetch(
            `${CONFIG.BACKENDS[this.backend].url}/api/results/${jobId}`
        );

        if (!response.ok) {
            const error = await response.json() as ErrorResponse;
            this.monitoring.trackError(error);
            throw new Error(error.message);
        }

        return response.json();
    }

    private stopPolling() {
        if (this.pollInterval) {
            clearInterval(this.pollInterval);
            this.pollInterval = undefined;
        }
    }
} 