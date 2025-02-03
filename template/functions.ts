import axios from 'axios'

export async function add(email: string, password: string): Promise<any>{
let data:any = [email, password];
 let response = await axios.post('http://localhost:3000/add', data);
 return response.data;
}

