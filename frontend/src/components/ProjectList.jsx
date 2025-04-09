import React from 'react';

export default function ProjectList({ projects }) {
  const copyToClipboard = (text) => {
    navigator.clipboard.writeText(text);
    alert('ID copiado para a área de transferência!');
  };

  return (
    <div>
      <h2>Projetos</h2>
      <ul>
        {projects.map(project => (
          <li key={project.id}>
            <strong>{project.title}</strong> — ✅ {project.votes_in_favor} | ❌ {project.votes_against} | 🧮 {project.total_votes}<br/>
            <strong>ID:</strong> {project.id}
            <button onClick={() => copyToClipboard(project.id)} style={{ marginLeft: '10px' }}>
              Copiar ID
            </button>
            <br />
          </li>
        ))}
      </ul>
    </div>
  );
}
