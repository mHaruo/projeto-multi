import React, { useState, useEffect } from 'react';
import { listUsers, getRanking } from './api/UserApi';
import { listProjects } from './api/ProjectApi';

import UserForm from './components/UserForm';
import UserList from './components/UserList';
import StarForm from './components/StarForm';
import UpdateUserForm from './components/UpdateUserForm';
import Ranking from './components/Ranking';
import ProjectForm from './components/ProjectForm';
import ProjectList from './components/ProjectList';
import VoteForm from './components/VoteForm';

export default function App() {
  const [users, setUsers] = useState([]);
  const [ranking, setRanking] = useState([]);
  const [projects, setProjects] = useState([]);

  const loadUsers = async () => {
    const res = await listUsers();
    setUsers(res.data);
  };

  const loadRanking = async () => {
    const res = await getRanking();
    setRanking(res.data);
  };

  const loadProjects = async () => {
    const res = await listProjects();
    setProjects(res.data);
  };

  useEffect(() => {
    loadUsers();
    loadRanking();
    loadProjects();
  }, []);

  return (
    <div style={{ padding: 20 }}>
      <UserForm onUserChange={loadUsers} />
      <UserList users={users} />
      <UpdateUserForm refresh={loadUsers} />
      <StarForm refresh={loadRanking} />
      <Ranking data={ranking} />
      <ProjectForm refresh={loadProjects} />
      <ProjectList projects={projects} />
      <VoteForm refresh={loadProjects} />
    </div>
  );
}
