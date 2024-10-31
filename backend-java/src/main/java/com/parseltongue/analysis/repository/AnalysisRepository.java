package com.parseltongue.analysis.repository;

import org.springframework.data.mongodb.repository.MongoRepository;
import org.springframework.data.mongodb.repository.Query;
import org.springframework.data.mongodb.repository.Aggregation;
import java.util.List;
import java.util.Optional;
import java.time.LocalDateTime;

@Repository
public interface AnalysisRepository extends MongoRepository<AnalysisJob, String> {
    @Query("{ 'backend': ?0 }")
    List<AnalysisJob> findByBackend(String backend);
    
    @Query("{ 'status': 'complete' }")
    List<AnalysisJob> findCompleted();
    
    @Query(value = "{ 'status': 'complete' }", sort = "{ 'processingTimeMs': 1 }")
    List<AnalysisJob> findFastest();

    @Query("{ 'status': ?0 }")
    List<AnalysisJob> findByStatus(String status);

    @Query("{ 'jobId': ?0, 'status': { $ne: 'complete' } }")
    Optional<AnalysisJob> findIncompleteJob(String jobId);

    @Query(value = "{ 'createdAt': { $lt: ?0 } }", delete = true)
    void deleteOldJobs(LocalDateTime cutoff);
} 