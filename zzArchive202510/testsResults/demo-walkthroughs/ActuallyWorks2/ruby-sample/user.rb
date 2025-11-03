# Ruby User class demonstrating OOP patterns
class User
  attr_reader :id, :name, :email
  attr_accessor :active

  def initialize(id, name, email)
    @id = id
    @name = name
    @email = email
    @active = true
  end

  def full_name
    @name.split(' ').map(&:capitalize).join(' ')
  end

  def deactivate
    @active = false
  end

  def self.find_by_email(email)
    # Stub for database query
    nil
  end
end

module Authentication
  def authenticate(password)
    # Simple authentication logic
    password == "secret123"
  end

  def generate_token
    SecureRandom.hex(32)
  end
end

class AdminUser < User
  include Authentication

  def initialize(id, name, email, permissions)
    super(id, name, email)
    @permissions = permissions
  end

  def has_permission?(action)
    @permissions.include?(action)
  end
end
