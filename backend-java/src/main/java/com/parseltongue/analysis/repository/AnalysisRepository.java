package com.parseltongue.analysis.repository;

import com.parseltongue.analysis.model.AnalysisJob;
import org.springframework.data.mongodb.repository.MongoRepository;
import org.springframework.stereotype.Repository;

import java.time.LocalDateTime;
import java.util.List;

@Repository
public interface AnalysisRepository extends MongoRepository<AnalysisJob, String> {
    // L2: Custom queries
    List<AnalysisJob> findByBackend(String backend);
    List<AnalysisJob> findByStatus(String status);

    // L3: Aggregation methods can be added here if needed

    // L4: Maintenance tasks
    void deleteByCreatedAtBefore(LocalDateTime cutoff);
} 