import React from 'react';

export default function Ranking({ data }) {
  return (
    <div>
      <h2>Ranking</h2>
      <ul>
        {data.map((user, index) => (
          <li key={index}>
            <strong>{user.name}</strong> — ⭐ {user.stars} — {user.badge}
          </li>
        ))}
      </ul>
    </div>
  );
}
