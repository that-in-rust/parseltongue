/**
 * Exception Pyramid:
 * L1: Core exception types
 * L2: Error categorization
 * L3: Recovery strategies
 * L4: Monitoring hooks
 */

@Getter
public class AnalysisException extends RuntimeException {
    private final String code;
    private final ErrorSeverity severity;
    private final boolean retryable;

    public AnalysisException(String message, String code, ErrorSeverity severity, boolean retryable) {
        super(message);
        this.code = code;
        this.severity = severity;
        this.retryable = retryable;
    }

    public enum ErrorSeverity {
        LOW, MEDIUM, HIGH, CRITICAL
    }
} 