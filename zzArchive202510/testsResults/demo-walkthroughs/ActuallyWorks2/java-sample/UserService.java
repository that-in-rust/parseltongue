package com.example.service;

import java.util.List;
import java.util.Optional;

/**
 * UserService handles user business logic
 */
public class UserService {
    private final UserRepository repository;
    private final EmailService emailService;

    public UserService(UserRepository repository, EmailService emailService) {
        this.repository = repository;
        this.emailService = emailService;
    }

    public Optional<User> findById(Long id) {
        return repository.findById(id);
    }

    public List<User> findAllActive() {
        return repository.findByStatus(UserStatus.ACTIVE);
    }

    public User createUser(String name, String email) {
        User user = new User(name, email);
        User saved = repository.save(user);
        emailService.sendWelcomeEmail(saved.getEmail());
        return saved;
    }

    public void deactivateUser(Long id) {
        repository.findById(id).ifPresent(user -> {
            user.setStatus(UserStatus.INACTIVE);
            repository.save(user);
        });
    }

    private boolean isValidEmail(String email) {
        return email != null && email.contains("@");
    }
}

interface UserRepository {
    Optional<User> findById(Long id);
    List<User> findByStatus(UserStatus status);
    User save(User user);
}

enum UserStatus {
    ACTIVE,
    INACTIVE,
    PENDING
}
