import { RequestManager, HTTPTransport, WebSocketTransport, Client } from "@open-rpc/client-js";
const httpTransport = new HTTPTransport("http://"+ window.location.host +"/api");
const socktransport = new WebSocketTransport("ws://"+ window.location.host + "/ws/api");
const manager = new RequestManager([socktransport, httpTransport], () => crypto.randomUUID());
const client = new Client(manager);

let deAuthenticatedCallback;

export function setup(_deAuthenticatedCallback: () => void) {
  deAuthenticatedCallback = _deAuthenticatedCallback;
}

export async function apiCall(method: string, params: Record<string, any>): Promise<any>{
  try {
    const result = await client.request({method, params});
    console.debug("api call result", result);
  } catch (ex){
    console.debug("api call epic fail", ex);
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

export async function checkAuthentication() {
  const res = await apiCall("session-check", {});
  if (res.error == "HTTP: Your Session cookie is invalid") return {auth: 0, error: null};
  if (res.error == "HTTP: Your Session Requires TFA") return {auth: 1, error: null};
  else if (res.error) return {auth: 0, error: res.error};
  else {
    /* TODO add commit_hash storing
    const last_hash = window.localStorage.getItem("commit_hash");

    if (last_hash) {
      if (last_hash !== res.data.commit_hash) {
        console.log("Detected New Backend Version, Reloading...");
        window.localStorage.removeItem("commit_hash");
        window.location.reload(true);
      }
    } else window.localStorage.setItem("commit_hash", res.data.commit_hash);
    */
    return {auth: 2, error: null};
  }
}