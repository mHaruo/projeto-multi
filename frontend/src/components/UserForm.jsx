import React, { useState } from 'react';
import { createUser, updateUser } from '../api/UserApi';

export default function UserForm({ refresh }) {
  const [newUser, setNewUser] = useState({ name: '', linkedin: '', github: '', twitter: '' });
  const [updateData, setUpdateData] = useState({ id: '', name: '', linkedin: '', github: '', twitter: '' });
  const handleCreate = async () => {
    await createUser(newUser);
    setNewUser({ name: '', linkedin: '', github: '', twitter: '' });
    if (typeof refresh === 'function') refresh();
  };

  const handleUpdate = async () => {
    const { id, ...rest } = updateData;
    await updateUser(id, rest);
    setUpdateData({ id: '', name: '', linkedin: '', github: '', twitter: '' });
    if (typeof refresh === 'function') refresh();
  };

  return (
    <div>
      <h3>Criar Usuário</h3>
      <input placeholder="Nome" value={newUser.name} onChange={e => setNewUser({ ...newUser, name: e.target.value })} />
      <input placeholder="LinkedIn" value={newUser.linkedin} onChange={e => setNewUser({ ...newUser, linkedin: e.target.value })} />
      <input placeholder="GitHub" value={newUser.github} onChange={e => setNewUser({ ...newUser, github: e.target.value })} />
      <input placeholder="Twitter" value={newUser.twitter} onChange={e => setNewUser({ ...newUser, twitter: e.target.value })} />
      <button onClick={handleCreate}>Criar</button>

      <h3>Atualizar Usuário</h3>
      <input placeholder="ID" value={updateData.id} onChange={e => setUpdateData({ ...updateData, id: e.target.value })} />
      <input placeholder="Novo nome" value={updateData.name} onChange={e => setUpdateData({ ...updateData, name: e.target.value })} />
      <input placeholder="LinkedIn" value={updateData.linkedin} onChange={e => setUpdateData({ ...updateData, linkedin: e.target.value })} />
      <input placeholder="GitHub" value={updateData.github} onChange={e => setUpdateData({ ...updateData, github: e.target.value })} />
      <input placeholder="Twitter" value={updateData.twitter} onChange={e => setUpdateData({ ...updateData, twitter: e.target.value })} />
      <button onClick={handleUpdate}>Atualizar</button>
    </div>
  );
}
