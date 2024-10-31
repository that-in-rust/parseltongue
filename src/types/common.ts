/**
 * Types Pyramid:
 * L1: Core type definitions
 * L2: State interfaces
 * L3: API responses
 * L4: Utility types
 */

// L1: Core types
export type Backend = 'java' | 'rust';

export interface AnalysisResult {
    jobId: string;
    backend: Backend;
    totalFiles: number;
    filesPerSecond: number;
    processingTimeMs: number;
    languageBreakdown: Record<string, number>;
    memoryUsage: number;
}

// L2: State interfaces
export interface AnalysisProgress {
    status: 'queued' | 'processing' | 'complete' | 'error';
    stage: 'cloning' | 'analyzing' | 'aggregating';
    currentFile?: string;
    progress: number;
}

// L3: API responses
export interface AnalysisResponse {
    jobId: string;
    status: string;
    backend: Backend;
}

export interface ErrorResponse {
    code: string;
    message: string;
    severity: 'LOW' | 'MEDIUM' | 'HIGH' | 'CRITICAL';
    retryable: boolean;
}

// L4: Utility types
export type ProgressCallback = (progress: AnalysisProgress) => void;
export type ErrorCallback = (error: ErrorResponse) => void; 