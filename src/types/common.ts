/**
 * Type System Pyramid:
 * L1: Core enums and constants
 * L2: Progress and status interfaces
 * L3: Results and metrics interfaces
 * L4: Job management interfaces
 */

// L1: Core Enums and Constants
export type Backend = 'java' | 'rust';
export type AnalysisStatus = 'queued' | 'processing' | 'complete' | 'error';
export type AnalysisStage = 'cloning' | 'scanning' | 'analyzing' | 'complete';

// L2: Progress Tracking
export interface ProgressUpdate {
    backend: Backend;
    stage: AnalysisStage;
    currentFile: string;
    progress: number;
    speed?: number;
    memoryUsage?: number;
}

// L3: Results and Metrics
export interface AnalysisResult {
    processingTimeMs: number;
    totalFiles: number;
    languageBreakdown: Record<string, number>;
    filesPerSecond: number;
    memoryUsage: number;
    timestamp: Date;
    backend: Backend;
}

// L4: Job Management
export interface AnalysisJob {
    jobId: string;
    status: AnalysisStatus;
    stage: AnalysisStage;
    currentFile: string;
    progress: number;
    results?: {
        java?: AnalysisResult;
        rust?: AnalysisResult;
    };
    error?: {
        message: string;
        code: string;
        timestamp: Date;
        retryCount?: number;
    };
    createdAt: Date;
    updatedAt: Date;
    performance?: {
        cpuUsage: number;
        memoryUsage: number;
        diskIO: number;
    };
}

// L4: Performance Monitoring
export interface PerformanceStats {
    avgProcessingTime: number;
    maxMemoryUsage: number;
    totalFilesProcessed: number;
    errorRate: number;
    successRate: number;
    backend: Backend;
} 