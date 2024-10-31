/**
 * Error Handler Pyramid:
 * L1: Core error responses
 * L2: Error categorization
 * L3: Recovery logic
 * L4: Monitoring integration
 */

@RestControllerAdvice
@Slf4j
public class GlobalExceptionHandler {
    private final MonitoringService monitoringService;

    @ExceptionHandler(AnalysisException.class)
    public ResponseEntity<ErrorResponse> handleAnalysisException(AnalysisException ex) {
        log.error("Analysis error: {}", ex.getMessage(), ex);
        monitoringService.trackError("analysis", ex);

        return ResponseEntity
            .status(HttpStatus.INTERNAL_SERVER_ERROR)
            .body(new ErrorResponse(
                ex.getCode(),
                ex.getMessage(),
                ex.getSeverity(),
                ex.isRetryable()
            ));
    }

    @ExceptionHandler(Exception.class)
    public ResponseEntity<ErrorResponse> handleGenericException(Exception ex) {
        log.error("Unexpected error: {}", ex.getMessage(), ex);
        monitoringService.trackError("system", ex);

        return ResponseEntity
            .status(HttpStatus.INTERNAL_SERVER_ERROR)
            .body(new ErrorResponse(
                "SYSTEM_ERROR",
                "An unexpected error occurred",
                ErrorSeverity.HIGH,
                true
            ));
    }

    @Data
    @AllArgsConstructor
    static class ErrorResponse {
        private String code;
        private String message;
        private ErrorSeverity severity;
        private boolean retryable;
    }
} 