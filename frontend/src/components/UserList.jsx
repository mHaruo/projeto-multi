import React from 'react';

export default function UserList({ users }) {
  const copyToClipboard = (text) => {
    navigator.clipboard.writeText(text);
    alert('ID copiado para a área de transferência!');
  };

  return (
    <div>
      <h2>Usuários</h2>
      <ul>
        {users.map(user => (
          <li key={user.id}>
            <strong>{user.name}</strong> — ⭐ {user.stars} <br />
            <strong>ID:</strong> {user.id}
            <button onClick={() => copyToClipboard(user.id)} style={{ marginLeft: '10px' }}>
              Copiar ID
            </button>
            <br />
          </li>
        ))}
      </ul>
    </div>
  );
}
