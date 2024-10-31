/**
 * Domain Model Pyramid:
 * L1: Core entity attributes
 * L2: Status tracking
 * L3: Results aggregation
 * L4: Audit fields
 */

@Data
@Document(collection = "analysis_jobs")
public class AnalysisJob {
    @Id
    private String jobId;
    private String backend = "java";
    private String status = "queued";
    private String stage = "cloning";
    private String currentFile;
    private double progress;
    private Map<String, Integer> languageBreakdown = new HashMap<>();
    private Long processingTimeMs;
    private Double filesPerSecond;
    private Integer totalFiles;
    
    @CreatedDate
    private LocalDateTime createdAt;
    
    @LastModifiedDate
    private LocalDateTime updatedAt;

    public AnalysisJob(String jobId) {
        this.jobId = jobId;
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
        this.filesPerSecond = this.totalFiles / (timeMs / 1000.0);
    }

    public void fail(String error) {
        this.status = "error";
    }
} 