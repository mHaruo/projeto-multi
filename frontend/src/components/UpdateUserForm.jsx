import React, { useState } from 'react';
import { updateUser } from '../api/UserApi';

export default function UpdateUserForm({ refresh }) {
  const [userId, setUserId] = useState('');
  const [name, setName] = useState('');
  const [linkedin, setLinkedin] = useState('');
  const [github, setGithub] = useState('');
  const [twitter, setTwitter] = useState('');
  const [message, setMessage] = useState('');

  const handleUpdate = async () => {
    try {
      await updateUser(userId, { name, linkedin, github, twitter });
      setMessage('Usuário atualizado com sucesso.');
      refresh(); 
    } catch {
      setMessage('Erro ao atualizar.');
    }
  };

  return (
    <div>
      <h2>Atualizar Usuário</h2>
      <input placeholder="ID" onChange={e => setUserId(e.target.value)} />
      <input placeholder="Nome" onChange={e => setName(e.target.value)} />
      <input placeholder="LinkedIn" onChange={e => setLinkedin(e.target.value)} />
      <input placeholder="GitHub" onChange={e => setGithub(e.target.value)} />
      <input placeholder="Twitter" onChange={e => setTwitter(e.target.value)} />
      <button onClick={handleUpdate}>Atualizar</button>
      {message && <p>{message}</p>}
    </div>
  );
}
