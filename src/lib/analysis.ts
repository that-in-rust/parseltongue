/**
 * Analysis Controller Pyramid:
 * L1: Core analysis orchestration
 * L2: Backend communication
 * L3: Progress tracking
 * L4: Error handling & metrics
 */

import { AnalysisProgress, AnalysisResult, Backend } from '../types/common';
import { MonitoringService } from './monitoring';
import { CONFIG } from '../config/constants';

export class AnalysisController {
  private jobId: string;
  private monitor: MonitoringService;
  private baseUrl: string;

  constructor(private backend: Backend) {
    this.baseUrl = CONFIG.BACKENDS[backend].URL;
    this.monitor = MonitoringService.getInstance();
  }

  async startAnalysis(): Promise<string> {
    const startTime = Date.now();
    try {
      const response = await fetch(`${this.baseUrl}/api/analyze`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' }
      });
      
      if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
      
      const { jobId } = await response.json();
      this.jobId = jobId;
      
      this.monitor.trackMetrics(jobId, {
        gitCloneTime: Date.now() - startTime,
        analysisTime: 0,
        filesPerSecond: 0,
        memoryUsage: 0,
        timestamp: new Date()
      });
      
      return jobId;
    } catch (error) {
      this.monitor.trackError(this.jobId, error as Error);
      throw error;
    }
  }

  async getProgress(): Promise<AnalysisProgress> {
    try {
      const response = await fetch(`${this.baseUrl}/api/status/${this.jobId}`);
      if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
      return response.json();
    } catch (error) {
      this.monitor.trackError(this.jobId, error as Error);
      throw error;
    }
  }

  async getResults(): Promise<AnalysisResult> {
    try {
      const response = await fetch(`${this.baseUrl}/api/results/${this.jobId}`);
      if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
      return response.json();
    } catch (error) {
      this.monitor.trackError(this.jobId, error as Error);
      throw error;
    }
  }
}

export class SequentialAnalysisController {
  private monitor: MonitoringService;
  private jobId?: string;

  constructor() {
    this.monitor = MonitoringService.getInstance();
  }
  
  async startAnalysis(): Promise<void> {
    const startTime = Date.now();

    try {
      // 1. Java Analysis
      const javaController = new AnalysisController('java');
      this.jobId = await javaController.startAnalysis();
      await this.waitForCompletion(javaController);
      
      // 2. Rust Analysis
      const rustController = new AnalysisController('rust');
      await rustController.startAnalysis();
      await this.waitForCompletion(rustController);

      this.monitor.trackMetrics(this.jobId, {
        analysisTime: Date.now() - startTime,
        gitCloneTime: 0,
        filesPerSecond: 0,
        memoryUsage: 0,
        timestamp: new Date()
      });
    } catch (error) {
      this.monitor.trackError(this.jobId || 'sequential', error as Error);
      throw error;
    }
  }

  private async waitForCompletion(controller: AnalysisController): Promise<void> {
    while (true) {
      const progress = await controller.getProgress();
      if (progress.status === 'complete') break;
      await new Promise(r => setTimeout(r, 1000));
    }
  }
} 