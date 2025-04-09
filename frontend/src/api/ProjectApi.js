import axios from 'axios';
const API_BASE = 'http://localhost:8000';

export const createProject = (data) => axios.post(`${API_BASE}/projects`, data);
export const listProjects = () => axios.get(`${API_BASE}/projects`);
export const voteProject = (projectId, data) => axios.post(`${API_BASE}/projects/vote/${projectId}`, data);
