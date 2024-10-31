package com.parseltongue.analysis.controller;

import com.parseltongue.analysis.model.AnalysisJob;
import com.parseltongue.analysis.service.AnalysisService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.concurrent.CompletableFuture;

@RestController
@RequestMapping("/api")
@Slf4j
@RequiredArgsConstructor
public class AnalysisController {

    private final AnalysisService analysisService;

    // L1: API endpoints
    @PostMapping("/analyze")
    public CompletableFuture<ResponseEntity<?>> startAnalysis() {
        // L3: Service delegation
        return analysisService.startAnalysis()
                .thenApply(jobId -> ResponseEntity.ok().body(new AnalysisResponse(jobId, "queued", "java")))
                .exceptionally(ex -> {
                    log.error("Failed to start analysis: {}", ex.getMessage());
                    return ResponseEntity.status(500).body("Failed to start analysis");
                });
    }

    @GetMapping("/status/{jobId}")
    public ResponseEntity<?> getStatus(@PathVariable String jobId) {
        try {
            // L3: Service delegation
            AnalysisJob job = analysisService.getJobStatus(jobId);
            return ResponseEntity.ok(job);
        } catch (Exception e) {
            // L4: Exception handling
            log.error("Error fetching status for job {}: {}", jobId, e.getMessage());
            return ResponseEntity.status(404).body("Job not found");
        }
    }

    // Additional endpoints like getResults can be added similarly

    // Inner class for response
    static class AnalysisResponse {
        public String jobId;
        public String status;
        public String backend;

        public AnalysisResponse(String jobId, String status, String backend) {
            this.jobId = jobId;
            this.status = status;
            this.backend = backend;
        }
    }
} 