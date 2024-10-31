/**
 * Monitoring Service Pyramid:
 * L1: Core metrics tracking
 * L2: Error aggregation
 * L3: Performance monitoring
 * L4: Analytics integration
 */

import { Backend, PerformanceStats } from '../types/common';
import { CONFIG } from '../config/constants';

interface MetricsPayload {
    gitCloneTime: number;
    analysisTime: number;
    filesPerSecond: number;
    memoryUsage: number;
    timestamp: Date;
}

export class MonitoringService {
    private static instance: MonitoringService;
    private errorCounts: Map<string, number> = new Map();
    private performanceStats: Map<Backend, PerformanceStats> = new Map();

    private constructor() {
        // Initialize with default stats
        this.performanceStats.set('java', {
            avgProcessingTime: 0,
            maxMemoryUsage: 0,
            totalFilesProcessed: 0,
            errorRate: 0,
            successRate: 100,
            backend: 'java'
        });
        this.performanceStats.set('rust', {
            avgProcessingTime: 0,
            maxMemoryUsage: 0,
            totalFilesProcessed: 0,
            errorRate: 0,
            successRate: 100,
            backend: 'rust'
        });
    }

    static getInstance(): MonitoringService {
        if (!MonitoringService.instance) {
            MonitoringService.instance = new MonitoringService();
        }
        return MonitoringService.instance;
    }

    trackMetrics(jobId: string, metrics: MetricsPayload): void {
        if (!CONFIG.FEATURES.enableMetricsLogging) return;

        console.info(`[Metrics] Job ${jobId}:`, {
            gitCloneTime: `${metrics.gitCloneTime}ms`,
            analysisTime: `${metrics.analysisTime}ms`,
            filesPerSecond: metrics.filesPerSecond,
            memoryUsage: `${Math.round(metrics.memoryUsage / 1024 / 1024)}MB`,
            timestamp: metrics.timestamp
        });
    }

    trackError(jobId: string, error: Error): void {
        const count = (this.errorCounts.get(error.name) || 0) + 1;
        this.errorCounts.set(error.name, count);

        console.error(`[Error] Job ${jobId}:`, {
            type: error.name,
            message: error.message,
            count,
            timestamp: new Date()
        });
    }

    updatePerformanceStats(backend: Backend, stats: Partial<PerformanceStats>): void {
        if (!CONFIG.FEATURES.enablePerformanceTracking) return;

        const currentStats = this.performanceStats.get(backend)!;
        this.performanceStats.set(backend, {
            ...currentStats,
            ...stats
        });
    }

    getPerformanceStats(backend: Backend): PerformanceStats {
        return this.performanceStats.get(backend)!;
    }

    getErrorStats(): Map<string, number> {
        return new Map(this.errorCounts);
    }
} 