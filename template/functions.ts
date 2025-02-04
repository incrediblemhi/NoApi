import axios from 'axios'

export async function add(email: string, password: string, _username: string): Promise<{
email: string;
    password: string;
}>{
let data:any = [email, password, _username];
 let response = await axios.post('http://localhost:3000/add', data);
 return response.data;
}

