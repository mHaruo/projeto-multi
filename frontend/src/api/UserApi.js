import axios from 'axios';
const API_BASE = 'http://localhost:8000';

export const createUser = (data) => axios.post(`${API_BASE}/users`, data);
export const listUsers = () => axios.get(`${API_BASE}/users`);
export const updateUser = (id, data) => axios.put(`${API_BASE}/users/${id}`, data);
export const giveStar = (fromId, toId) => axios.post(`${API_BASE}/users/${fromId}/star/${toId}`);
export const getRanking = () => axios.get(`${API_BASE}/ranking`);
