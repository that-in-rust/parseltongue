package com.parseltongue.analysis.repository;

import org.springframework.data.mongodb.repository.MongoRepository;
import org.springframework.data.mongodb.repository.Query;
import org.springframework.data.mongodb.repository.Aggregation;
import java.util.List;
import java.util.Optional;
import java.time.LocalDateTime;

@Repository
public interface AnalysisRepository extends MongoRepository<AnalysisJob, String> {
    @Aggregation(pipeline = {
        "{ $match: { 'status': 'complete' } }",
        "{ $group: { " +
            "_id: '$backend', " +
            "avgTime: { $avg: '$result.processingTimeMs' }, " +
            "totalFiles: { $sum: '$result.totalFiles' }, " +
            "avgMemory: { $avg: '$result.memoryUsage' }" +
        "} }",
        "{ $project: { " +
            "backend: '$_id', " +
            "avgTime: 1, " +
            "totalFiles: 1, " +
            "avgMemory: 1, " +
            "_id: 0" +
        "} }"
    })
    List<BackendPerformanceStats> getBackendPerformanceStats();

    @Query("{ 'status': ?0 }")
    List<AnalysisJob> findByStatus(String status);

    @Query("{ 'jobId': ?0, 'status': { $ne: 'complete' } }")
    Optional<AnalysisJob> findIncompleteJob(String jobId);

    @Query(value = "{ 'createdAt': { $lt: ?0 } }", delete = true)
    void deleteOldJobs(LocalDateTime cutoff);
} 