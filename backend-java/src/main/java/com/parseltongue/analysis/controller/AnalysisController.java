package com.parseltongue.analysis.controller;

import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.Map;
import java.util.concurrent.CompletableFuture;

import lombok.extern.slf4j.Slf4j;

@RestController
@RequestMapping("/api")
@Slf4j
public class AnalysisController {
    private final AnalysisService analysisService;
    private final AnalysisRepository repository;
    private final MonitoringService monitoringService;

    @PostMapping("/analyze")
    public CompletableFuture<ResponseEntity<Map<String, String>>> startAnalysis() {
        return analysisService.startAnalysis()
            .thenApply(jobId -> ResponseEntity.ok(Map.of(
                "jobId", jobId,
                "status", "queued",
                "backend", "java"
            )))
            .exceptionally(e -> {
                log.error("Analysis failed", e);
                monitoringService.trackError(e);
                return ResponseEntity.status(500)
                    .body(Map.of("error", e.getMessage()));
            });
    }

    @GetMapping("/status/{jobId}")
    public ResponseEntity<AnalysisProgress> getStatus(@PathVariable String jobId) {
        try {
            return repository.findById(jobId)
                .map(job -> ResponseEntity.ok(mapToProgress(job)))
                .orElse(ResponseEntity.notFound().build());
        } catch (Exception e) {
            log.error("Error fetching status", e);
            monitoringService.trackError(e);
            throw e;
        }
    }

    @GetMapping("/results/{jobId}")
    public ResponseEntity<AnalysisResult> getResults(@PathVariable String jobId) {
        try {
            return repository.findById(jobId)
                .map(job -> ResponseEntity.ok(job.getResult()))
                .orElse(ResponseEntity.notFound().build());
        } catch (Exception e) {
            log.error("Error fetching results", e);
            monitoringService.trackError(e);
            throw e;
        }
    }

    private AnalysisProgress mapToProgress(AnalysisJob job) {
        return new AnalysisProgress(
            job.getStatus(),
            job.getStage(),
            job.getCurrentFile(),
            job.getProgress()
        );
    }
} 