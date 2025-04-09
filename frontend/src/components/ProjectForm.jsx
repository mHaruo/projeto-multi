import React, { useState } from 'react';
import { createProject } from '../api/ProjectApi';

export default function ProjectForm({ refresh }) {
  const [title, setTitle] = useState('');

  const handleCreate = async () => {
    await createProject({ title });
    setTitle('');
    refresh();
  };

  return (
    <div>
      <h2>Criar Projeto</h2>
      <input
        placeholder="Título"
        value={title}
        onChange={e => setTitle(e.target.value)}
      />
      <button onClick={handleCreate}>Criar</button>
    </div>
  );
}
