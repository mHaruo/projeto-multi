import React, { useState } from 'react';
import { voteProject } from '../api/ProjectApi';

export default function VoteForm({ refresh }) {
  const [projectId, setProjectId] = useState('');
  const [userId, setUserId] = useState('');
  const [vote, setVote] = useState('sim');

  const handleVote = async () => {
    await voteProject(projectId, { user_id: userId, vote });
    refresh();
  };

  return (
    <div>
      <h2>Votar em Projeto</h2>
      <input
        placeholder="ID do Projeto"
        value={projectId}
        onChange={e => setProjectId(e.target.value)}
      />
      <input
        placeholder="ID do Usuário"
        value={userId}
        onChange={e => setUserId(e.target.value)}
      />
      <select value={vote} onChange={e => setVote(e.target.value)}>
        <option value="sim">Sim</option>
        <option value="nao">Não</option>
      </select>
      <button onClick={handleVote}>Votar</button>
    </div>
  );
}
