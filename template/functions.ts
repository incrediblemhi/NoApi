import axios from 'axios'

export interface User {
     email: string;
     password: string;
}

export async function create_user(email: string, password: string, _username: string): Promise<User>{
let base_url = window.origin;
let data:any = [email, password, _username];
 let response = await axios.post(`${base_url}/create_user`, data);
 return response.data;
}

