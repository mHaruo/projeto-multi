import React, { useState } from 'react';
import { giveStar } from '../api/UserApi';

export default function StarForm({ refresh }) {
  const [from, setFrom] = useState('');
  const [to, setTo] = useState('');
  const [message, setMessage] = useState('');

  const handleGiveStar = async () => {
    try {
      await giveStar(from, to);
      setMessage('Estrela doada com sucesso!');
      if (typeof refresh === 'function') refresh();
    } catch (error) {
      const errorMsg = error?.response?.data || 'Erro ao doar estrela';
      setMessage(String(errorMsg)); // 🔒 garantir que sempre será string
    }
  };

  return (
    <div>
      <h2>Doar Estrelas</h2>
      <input placeholder="De (ID)" onChange={e => setFrom(e.target.value)} />
      <input placeholder="Para (ID)" onChange={e => setTo(e.target.value)} />
      <button onClick={handleGiveStar}>Doar</button>
      {message && <p>{message}</p>}
    </div>
  );
}
