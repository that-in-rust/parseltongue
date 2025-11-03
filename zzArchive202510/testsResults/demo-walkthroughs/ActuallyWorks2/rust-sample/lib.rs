use std::collections::HashMap;

/// User struct with basic fields
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    active: bool,
}

impl User {
    pub fn new(id: u64, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

pub trait Repository<T> {
    fn find_by_id(&self, id: u64) -> Option<&T>;
    fn save(&mut self, item: T) -> Result<u64, RepositoryError>;
    fn delete(&mut self, id: u64) -> Result<(), RepositoryError>;
}

pub struct UserRepository {
    storage: HashMap<u64, User>,
    next_id: u64,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Repository<User> for UserRepository {
    fn find_by_id(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn save(&mut self, mut user: User) -> Result<u64, RepositoryError> {
        if user.id == 0 {
            user.id = self.next_id;
            self.next_id += 1;
        }
        let id = user.id;
        self.storage.insert(id, user);
        Ok(id)
    }

    fn delete(&mut self, id: u64) -> Result<(), RepositoryError> {
        self.storage.remove(&id)
            .map(|_| ())
            .ok_or(RepositoryError::NotFound)
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    InvalidData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
        assert_eq!(user.id, 1);
        assert!(user.is_active());
    }
}
