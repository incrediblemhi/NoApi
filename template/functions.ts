import axios from 'axios'

export async function hello(num1: number, num2: number): Promise<number>{
let data:any = [parseInt(num1.toString(), 10), parseInt(num2.toString(), 10)];
 let response = await axios.post('http://localhost:3000/hello', data);
 return response.data;
}

