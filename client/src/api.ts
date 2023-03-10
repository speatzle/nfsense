import { RequestManager, HTTPTransport, WebSocketTransport, Client } from "@open-rpc/client-js";
import axios from "axios";
const httpTransport = new HTTPTransport("http://"+ window.location.host +"/api");
// const socktransport = new WebSocketTransport("ws://"+ window.location.host + "/ws/api");
const manager = new RequestManager([httpTransport], () => crypto.randomUUID());
const client = new Client(manager);

let deAuthenticatedCallback;

export function setup(_deAuthenticatedCallback: () => void) {
  deAuthenticatedCallback = _deAuthenticatedCallback;
}

export async function apiCall(method: string, params: Record<string, any>): Promise<any>{
  try {
    const result = await client.request({method, params});
    console.debug("api call result", result);
    return { Data: result, Error: null};
  } catch (ex){
    console.debug("api call epic fail", ex);
    return { Data: null, Error: ex};
  }
}

export async function authenticate(username: string, password: string): Promise<any> {
  const pResponse = axios.post("/login", { username, password }, {timeout: 10100});
  try {
    const response = await pResponse;
    // Dont log this as the user password is inside: console.debug(response);
    return { data: response.data, error: null};
  } catch (error) {
    return { data: null, error: error};
  }
}

export async function logout(): Promise<any> {
  const pResponse = axios.post("/logout", null, {timeout: 10100});
  try {
    const response = await pResponse;
    return { data: response.data, error: null};
  } catch (error) {
    return { data: null, error: error};
  }
}

export async function checkAuthentication() {
  const pResponse = axios.post("/session", null, {timeout: 10100});
  try {
    const response = await pResponse;
    const last_hash = window.localStorage.getItem("commit_hash");

    if (last_hash) {
      if (last_hash !== response.data.commit_hash) {
        console.log("Detected New Backend Version, Reloading...");
        window.localStorage.removeItem("commit_hash");
        window.location.reload();
      }
    } else window.localStorage.setItem("commit_hash", response.data.commit_hash);
    return {auth: 2, error: null};
  } catch (error) {
    return {auth: 0, error: error};
  }
}