/**
 * Domain Model Pyramid:
 * L1: Core entity attributes
 * L2: Status and progress tracking
 * L3: Results and metrics
 * L4: Audit and performance data
 */

@Data
@Document(collection = "analysis_jobs")
public class AnalysisJob {
    // L1: Core entity
    @Id
    private String jobId;
    private String backend = "java";
    
    // L2: Status tracking
    @NotNull
    private String status;
    private String stage;
    private String currentFile;
    private double progress;
    
    // L3: Analysis results
    private AnalysisResult result;
    private Map<String, Integer> languageBreakdown;
    
    // L4: Performance metrics
    private Long memoryUsage;
    private Double filesPerSecond;
    private Long processingTimeMs;
    
    // Audit fields
    @CreatedDate
    private LocalDateTime createdAt;
    @LastModifiedDate
    private LocalDateTime updatedAt;
    
    public AnalysisJob(String jobId) {
        this.jobId = jobId;
        this.status = "queued";
        this.progress = 0.0;
        this.stage = "cloning";
        this.languageBreakdown = new HashMap<>();
    }

    public void updateProgress(String currentFile, double progress) {
        this.currentFile = currentFile;
        this.progress = progress;
        this.updatedAt = LocalDateTime.now();
    }

    public void complete(AnalysisResult result) {
        this.status = "complete";
        this.progress = 100.0;
        this.result = result;
        this.updatedAt = LocalDateTime.now();
    }

    public void fail(String error) {
        this.status = "error";
        this.updatedAt = LocalDateTime.now();
    }
} 