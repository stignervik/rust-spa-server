import Axios from 'axios';

export const http = Axios.create({
  baseURL: import.meta.env.VITE_BASE_URL,
});
