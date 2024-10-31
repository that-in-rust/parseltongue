/**
 * Domain Model Pyramid:
 * L1: Core entity attributes
 * L2: Status tracking
 * L3: Results aggregation
 * L4: Audit fields
 */

package com.parseltongue.analysis.model;

import lombok.Data;
import org.springframework.data.annotation.Id;
import org.springframework.data.mongodb.core.mapping.Document;
import org.springframework.data.annotation.CreatedDate;
import org.springframework.data.annotation.LastModifiedDate;

import java.time.LocalDateTime;
import java.util.HashMap;
import java.util.Map;

@Data
@Document(collection = "analysis_jobs")
public class AnalysisJob {
    // L1: Core entity attributes
    @Id
    private String jobId;
    private String backend;

    // L2: Status tracking
    private String status;
    private String stage;
    private String currentFile;
    private double progress;

    // L3: Results aggregation
    private Map<String, Integer> languageBreakdown = new HashMap<>();
    private Long processingTimeMs;
    private Double filesPerSecond;
    private Integer totalFiles;

    // L4: Audit fields
    @CreatedDate
    private LocalDateTime createdAt;
    @LastModifiedDate
    private LocalDateTime updatedAt;

    public AnalysisJob(String jobId, String backend) {
        this.jobId = jobId;
        this.backend = backend;
        this.status = "queued";
        this.stage = "initializing";
        this.progress = 0.0;
    }

    public void updateProgress(String currentFile, double progress) {
        this.currentFile = currentFile;
        this.progress = progress;
    }

    public void complete(Map<String, Integer> breakdown, long timeMs) {
        this.status = "complete";
        this.progress = 100.0;
        this.languageBreakdown = breakdown;
        this.processingTimeMs = timeMs;
        this.totalFiles = breakdown.values().stream().mapToInt(Integer::intValue).sum();
        this.filesPerSecond = totalFiles / (timeMs / 1000.0);
    }

    public void fail(String error) {
        this.status = "error";
        this.stage = error;
    }
} 