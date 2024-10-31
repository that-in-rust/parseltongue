/**
 * Service Layer Pyramid:
 * L1: Core analysis orchestration
 * L2: File processing
 * L3: Language detection
 * L4: Progress tracking
 */

@Service
@Slf4j
@RequiredArgsConstructor
public class AnalysisService {
    private final AnalysisRepository repository;
    private final AnalysisConfig config;

    @Async
    public CompletableFuture<String> startAnalysis() {
        String jobId = UUID.randomUUID().toString();
        AnalysisJob job = new AnalysisJob(jobId);
        repository.save(job);
        
        return CompletableFuture.supplyAsync(() -> {
            long startTime = System.currentTimeMillis();
            Map<String, Integer> breakdown = new HashMap<>();
            
            try {
                Path repoPath = Paths.get(config.getCacheDir(), jobId);
                processDirectory(repoPath, job, breakdown);
                
                job.complete(breakdown, System.currentTimeMillis() - startTime);
                repository.save(job);
                return jobId;
                
            } catch (Exception e) {
                job.fail(e.getMessage());
                repository.save(job);
                throw new AnalysisException("Analysis failed", "ANALYSIS_ERROR", 
                    ErrorSeverity.HIGH, true);
            }
        });
    }

    private void processDirectory(Path dir, AnalysisJob job, Map<String, Integer> breakdown) 
            throws IOException {
        try (Stream<Path> paths = Files.walk(dir)) {
            paths.filter(Files::isRegularFile)
                .forEach(path -> {
                    String ext = getExtension(path);
                    breakdown.merge(ext, 1, Integer::sum);
                    
                    job.updateProgress(
                        path.getFileName().toString(),
                        (breakdown.values().stream().mapToInt(Integer::intValue).sum() * 100.0) / 1000
                    );
                    repository.save(job);
                });
        }
    }

    private String getExtension(Path path) {
        String name = path.getFileName().toString();
        int dot = name.lastIndexOf('.');
        return dot > 0 ? name.substring(dot + 1) : "unknown";
    }
} 