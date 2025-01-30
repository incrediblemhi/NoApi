import axios from 'axios'

export async function add(email: string, password: string): Promise<any>{
let data:any = [];
 let response = await axios.post('http://localhost:3000/add', data);
 return response.data;
}

