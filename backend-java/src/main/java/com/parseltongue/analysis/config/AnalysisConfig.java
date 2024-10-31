package com.parseltongue.analysis.config;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.data.mongodb.repository.config.EnableMongoRepositories;
import org.springframework.scheduling.annotation.EnableAsync;
import org.springframework.validation.annotation.Validated;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.concurrent.Executor;

import org.springframework.data.mongodb.core.MongoTemplate;
import com.mongodb.client.MongoClients;

import javax.annotation.PostConstruct;

@Configuration
@EnableMongoRepositories
@EnableAsync
@Validated
public class AnalysisConfig {
    @Value("${analysis.repo.url:https://github.com/facebook/react.git}")
    private String repoUrl;
    
    @Value("${analysis.cache.dir:/tmp/parseltongue-cache}")
    private String cacheDir;
    
    @Value("${spring.data.mongodb.uri:mongodb://localhost:27017/parseltongue}")
    private String mongoUri;
    
    @Value("${server.port:8080}")
    private int serverPort;

    @Bean
    public Executor taskExecutor() {
        ThreadPoolTaskExecutor executor = new ThreadPoolTaskExecutor();
        executor.setCorePoolSize(2);
        executor.setMaxPoolSize(4);
        executor.setQueueCapacity(500);
        executor.setThreadNamePrefix("Analysis-");
        executor.initialize();
        return executor;
    }

    @Bean
    public MongoTemplate mongoTemplate() throws Exception {
        return new MongoTemplate(MongoClients.create(mongoUri), "parseltongue");
    }

    @PostConstruct
    public void validateConfig() throws IOException {
        Path cachePath = Paths.get(cacheDir);
        if (!Files.exists(cachePath)) {
            Files.createDirectories(cachePath);
        }
    }

    // Getters
    public String getRepoUrl() { return repoUrl; }
    public String getCacheDir() { return cacheDir; }
    public String getMongoUri() { return mongoUri; }
    public int getServerPort() { return serverPort; }
} 