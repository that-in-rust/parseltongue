/**
 * Service Layer Pyramid:
 * L1: Core analysis orchestration
 * L2: Git operations and file processing
 * L3: Metrics collection
 * L4: Error handling and recovery
 */

@Service
@Slf4j
@RequiredArgsConstructor
public class AnalysisService {
    private final AnalysisRepository repository;
    private final AnalysisConfig config;
    private final MonitoringService monitoring;

    @Async
    public CompletableFuture<String> startAnalysis() {
        String jobId = UUID.randomUUID().toString();
        AnalysisJob job = new AnalysisJob(jobId);
        repository.save(job);

        return CompletableFuture.supplyAsync(() -> {
            try {
                // L1: Core orchestration
                processAnalysis(job);
                return jobId;
            } catch (Exception e) {
                handleError(job, e);
                throw new RuntimeException(e);
            }
        });
    }

    private void processAnalysis(AnalysisJob job) throws Exception {
        // L2: Git operations
        cloneRepository(job);
        
        // L2: File processing
        processFiles(job);
        
        // L3: Metrics collection
        collectMetrics(job);
        
        // L4: Finalization
        completeJob(job);
    }

    private void cloneRepository(AnalysisJob job) throws GitAPIException {
        Path repoPath = Paths.get(config.getCacheDir(), "repos", job.getJobId());
        if (!Files.exists(repoPath)) {
            Git.cloneRepository()
                .setURI(config.getRepoUrl())
                .setDirectory(repoPath.toFile())
                .setProgressMonitor(new GitProgressMonitor(job))
                .call();
        }
    }

    private void processFiles(AnalysisJob job) throws IOException {
        AtomicLong fileCount = new AtomicLong(0);
        Path repoPath = Paths.get(config.getCacheDir(), "repos", job.getJobId());
        
        try (Stream<Path> files = Files.walk(repoPath)) {
            files.filter(Files::isRegularFile)
                .forEach(file -> {
                    processFile(job, file);
                    updateProgress(job, fileCount.incrementAndGet(), file);
                });
        }
    }

    private void collectMetrics(AnalysisJob job) {
        AnalysisResult result = new AnalysisResult();
        result.setProcessingTimeMs(System.currentTimeMillis() - job.getCreatedAt().toInstant().toEpochMilli());
        result.setTotalFiles(job.getTotalFiles());
        result.setFilesPerSecond((double) job.getTotalFiles() / (result.getProcessingTimeMs() / 1000.0));
        result.setMemoryUsage(Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory());
        
        monitoring.trackMetrics(job.getJobId(), result);
        job.setResult(result);
        repository.save(job);
    }

    @Transactional
    private void completeJob(AnalysisJob job) {
        job.setStatus("complete");
        job.setProgress(100.0);
        job.setStage("complete");
        repository.save(job);
        monitoring.recordCompletion(job.getJobId());
    }

    private void handleError(AnalysisJob job, Exception e) {
        log.error("Analysis failed for job {}: {}", job.getJobId(), e.getMessage(), e);
        monitoring.trackError(job.getJobId(), e);
        
        job.setStatus("error");
        job.setError(new AnalysisError(e));
        repository.save(job);
    }
} 