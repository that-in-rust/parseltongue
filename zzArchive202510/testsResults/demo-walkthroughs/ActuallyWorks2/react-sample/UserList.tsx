import React, { useState, useEffect } from 'react';

interface User {
  id: number;
  name: string;
  email: string;
  active: boolean;
}

interface UserListProps {
  initialUsers?: User[];
  onUserSelect?: (user: User) => void;
}

export const UserList: React.FC<UserListProps> = ({ initialUsers = [], onUserSelect }) => {
  const [users, setUsers] = useState<User[]>(initialUsers);
  const [loading, setLoading] = useState(false);
  const [filter, setFilter] = useState('');

  useEffect(() => {
    fetchUsers();
  }, []);

  const fetchUsers = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/users');
      const data = await response.json();
      setUsers(data);
    } catch (error) {
      console.error('Failed to fetch users:', error);
    } finally {
      setLoading(false);
    }
  };

  const filteredUsers = users.filter(user =>
    user.name.toLowerCase().includes(filter.toLowerCase()) ||
    user.email.toLowerCase().includes(filter.toLowerCase())
  );

  const handleUserClick = (user: User) => {
    onUserSelect?.(user);
  };

  const deactivateUser = async (userId: number) => {
    try {
      await fetch(`/api/users/${userId}/deactivate`, { method: 'POST' });
      setUsers(users.map(u =>
        u.id === userId ? { ...u, active: false } : u
      ));
    } catch (error) {
      console.error('Failed to deactivate user:', error);
    }
  };

  return (
    <div className="user-list">
      <input
        type="text"
        placeholder="Filter users..."
        value={filter}
        onChange={(e) => setFilter(e.target.value)}
      />

      {loading && <div>Loading...</div>}

      <ul>
        {filteredUsers.map(user => (
          <UserListItem
            key={user.id}
            user={user}
            onClick={() => handleUserClick(user)}
            onDeactivate={() => deactivateUser(user.id)}
          />
        ))}
      </ul>
    </div>
  );
};

interface UserListItemProps {
  user: User;
  onClick: () => void;
  onDeactivate: () => void;
}

const UserListItem: React.FC<UserListItemProps> = ({ user, onClick, onDeactivate }) => {
  return (
    <li onClick={onClick} className={user.active ? 'active' : 'inactive'}>
      <span>{user.name}</span>
      <span>{user.email}</span>
      {user.active && (
        <button onClick={(e) => {
          e.stopPropagation();
          onDeactivate();
        }}>
          Deactivate
        </button>
      )}
    </li>
  );
};
