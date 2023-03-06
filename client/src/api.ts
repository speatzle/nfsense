import { RequestManager, HTTPTransport, WebSocketTransport, Client } from "@open-rpc/client-js";
const httpTransport = new HTTPTransport("http://"+ window.location.host +"/api");
const socktransport = new WebSocketTransport("ws://"+ window.location.host + "/ws/api");
const manager = new RequestManager([socktransport, httpTransport], () => crypto.randomUUID());
const client = new Client(manager);

export async function apiCall(method: string, params: Record<string, any>){
  try {
    const result = await client.request({method, params});
    console.debug("api call result", result);
  } catch (ex){
    console.debug("api call epic fail", ex);
  }
}