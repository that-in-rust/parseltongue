/**
 * Service Layer Pyramid:
 * L1: Core analysis orchestration
 * L2: Repository interactions
 * L3: File processing
 * L4: Progress updates
 */

package com.parseltongue.analysis.service;

import com.parseltongue.analysis.config.AnalysisConfig;
import com.parseltongue.analysis.exception.AnalysisException;
import com.parseltongue.analysis.model.AnalysisJob;
import com.parseltongue.analysis.repository.AnalysisRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Service;

import java.io.IOException;
import java.nio.file.*;
import java.util.HashMap;
import java.util.Map;
import java.util.UUID;
import java.util.concurrent.CompletableFuture;
import java.util.stream.Stream;

@Service
@Slf4j
@RequiredArgsConstructor
public class AnalysisService {

    private final AnalysisRepository repository;
    private final AnalysisConfig config;

    // L1: Core analysis orchestration
    @Async
    public CompletableFuture<String> startAnalysis() {
        String jobId = UUID.randomUUID().toString();
        AnalysisJob job = new AnalysisJob(jobId, "java");
        repository.save(job);

        return CompletableFuture.runAsync(() -> {
            try {
                // L2: Repository interactions
                Path repoPath = cloneRepository(jobId);

                // L3: File processing
                Map<String, Integer> breakdown = processFiles(repoPath, job);

                // L4: Progress updates
                long processingTimeMs = System.currentTimeMillis() - job.getCreatedAt().atZone(java.time.ZoneId.systemDefault()).toInstant().toEpochMilli();
                job.complete(breakdown, processingTimeMs);
                repository.save(job);
            } catch (Exception e) {
                job.fail(e.getMessage());
                repository.save(job);
                log.error("Analysis failed for job {}: {}", jobId, e.getMessage());
                throw new AnalysisException("Analysis failed", "ANALYSIS_ERROR", e);
            }
        }).thenApply(v -> jobId);
    }

    private Path cloneRepository(String jobId) throws IOException {
        Path repoPath = Paths.get(config.getCacheDir(), "repo-" + jobId);
        if (!Files.exists(repoPath)) {
            Files.createDirectories(repoPath);
            // Clone logic would be implemented here
        }
        return repoPath;
    }

    private Map<String, Integer> processFiles(Path repoPath, AnalysisJob job) throws IOException {
        Map<String, Integer> languageBreakdown = new HashMap<>();
        try (Stream<Path> paths = Files.walk(repoPath)) {
            long totalFiles = 0;
            for (Path path : (Iterable<Path>) paths.filter(Files::isRegularFile)::iterator) {
                String ext = getFileExtension(path);
                languageBreakdown.merge(ext, 1, Integer::sum);
                totalFiles++;

                double progress = (double) totalFiles / 1000 * 100; // Simplified progress calculation
                job.updateProgress(path.getFileName().toString(), progress);
                repository.save(job);
            }
        }
        return languageBreakdown;
    }

    private String getFileExtension(Path path) {
        String filename = path.getFileName().toString();
        int index = filename.lastIndexOf('.');
        return index > 0 ? filename.substring(index + 1) : "unknown";
    }

    public AnalysisJob getJobStatus(String jobId) {
        return repository.findById(jobId).orElseThrow(() -> new AnalysisException("Job not found", "NOT_FOUND"));
    }
} 