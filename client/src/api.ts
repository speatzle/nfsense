// import WebSocketServer  from 'ws';
import JsonRPC from 'simple-jsonrpc-js';
import axios from 'axios';
import { useToast } from 'vue-toast-notification';

const $toast = useToast();

let jrpc = new JsonRPC.connect_xhr('/api');
// let socket = new WebSocket("ws://"+ window.location.host +"/ws/api");

let UnauthorizedCallback: Function;

export function setup(_UnauthorizedCallback: () => void) {
  UnauthorizedCallback = _UnauthorizedCallback;
}

export async function apiCall(method: string, params: Record<string, any>): Promise<any>{
  try {
    const pResult = jrpc.call(method, params);
    console.debug('[API] Calling ', method, params, pResult);
    const result = await pResult;
    console.debug('[API] Response', method, result);
    return { Data: result, Error: null };
  } catch (ex: any){
    if (ex.code === 401) {
      UnauthorizedCallback();
    } else {
      $toast.error(`${method }: ${  ex.message}`);
      console.debug('[API] Error   ', method, ex);
    }
    return { Data: null, Error: ex };
  }
}

export async function authenticate(username: string, password: string): Promise<any> {
  const pResponse = axios.post('/login', { username, password }, { timeout: 10100 });
  try {
    const response = await pResponse;
    // Dont log this as the user password is inside: console.debug(response);
    return { data: response.data, error: null };
  } catch (error) {
    return { data: null, error: error };
  }
}

export async function logout(): Promise<any> {
  const pResponse = axios.post('/logout', null, { timeout: 10100 });
  try {
    const response = await pResponse;
    return { data: response.data, error: null };
  } catch (error) {
    return { data: null, error: error };
  }
}

export async function checkAuthentication() {
  const pResponse = axios.post('/session', null, { timeout: 10100 });
  try {
    const response = await pResponse;
    const last_hash = window.localStorage.getItem('commit_hash');
    if (last_hash) {
      if (last_hash !== response.data.commit_hash) {
        console.log(`Detected New Backend Version ${response.data.commit_hash}, Reloading...`);
        window.localStorage.removeItem('commit_hash');
        window.location.reload();
      }
    } else window.localStorage.setItem('commit_hash', response.data.commit_hash);
    return { auth: 2, error: null };
  } catch (error: any) {
    if (error.response.status == 401) {
      return { auth: 0, error: null };
    }
    return { auth: 0, error: error };
  }
}