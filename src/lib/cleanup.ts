/**
 * Cleanup Service Pyramid:
 * L1: Core cleanup operations
 * L2: Scheduling
 * L3: Error handling
 * L4: Resource management
 */

import { promises as fs } from 'fs';
import { glob } from 'glob-promise';
import checkDiskSpace from 'check-disk-space';
import { MonitoringService } from './monitoring';

export class CleanupService {
    private static readonly MAX_AGE_MS = 24 * 60 * 60 * 1000; // 24 hours
    private static readonly MIN_DISK_SPACE = 1024 * 1024 * 1024; // 1GB
    private static readonly CACHE_DIR = process.env.CACHE_DIR || '/tmp/parseltongue-cache';
    private static monitor = MonitoringService.getInstance();

    static async cleanupOldResults(): Promise<void> {
        try {
            const now = Date.now();
            const analysisDir = `${this.CACHE_DIR}/analysis-*`;
            
            const dirs = await glob(analysisDir);
            for (const dir of dirs) {
                const stats = await fs.stat(dir);
                if (now - stats.mtimeMs > this.MAX_AGE_MS) {
                    await fs.rm(dir, { recursive: true, force: true });
                }
            }
        } catch (error) {
            this.monitor.trackError('cleanup', error as Error);
            throw error;
        }
    }

    static async ensureResources(): Promise<boolean> {
        try {
            const { free } = await checkDiskSpace(this.CACHE_DIR);
            return free > this.MIN_DISK_SPACE;
        } catch (error) {
            this.monitor.trackError('disk-check', error as Error);
            return false;
        }
    }
} 