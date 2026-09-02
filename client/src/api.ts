// import WebSocketServer  from 'ws';
import JsonRPC from "simple-jsonrpc-js";
import { useToast } from "vue-toast-notification";

const toast = useToast();

const jrpc = new JsonRPC.connect_xhr("/api");
// let socket = new WebSocket("ws://"+ window.location.host +"/ws/api");

let UnauthorizedCallback: () => void;

export function setup(_UnauthorizedCallback: () => void) {
  UnauthorizedCallback = _UnauthorizedCallback;
}

let $pendingChangeCount = 0;
export function usePendingChangeCount() {
  return $$($pendingChangeCount);
}

export async function apiCall(method: string, params: Record<string, any>): Promise<any> {
  try {
    const pResult = jrpc.call(method, params);
    console.debug("[API] Calling ", method, params, pResult);
    const result = await pResult;
    console.debug("[API] Response", method, result);

    // TODO: Replace with websocket later
    if (["config.pending.apply", "config.pending.discard", "config.pending.set"].includes(method)
      || [".create", ".update", ".delete"].some(suffix => method.endsWith(suffix))) {
      try {
        const res = await apiCall("config.pending.log", {});
        if (res.Error === null) {
          const changelog = res.Data;
          $pendingChangeCount = changelog.length;
        } else console.debug("error fetching pending log", res);
      } catch (ex: any) {
        console.debug("error fetching pending log", ex)
      }
    }

    return { Data: result, Error: null };
  } catch (ex: any) {
    if (ex.code === 401) UnauthorizedCallback();
    else {
      toast.error(`${method}: ${ex.message}`);
      console.debug("[API] Error   ", method, ex);
    }
    return { Data: null, Error: ex };
  }
}

const headers = { "Content-Type": "application/json" };

export async function authenticate(username: string, password: string): Promise<any> {
  try {
    const body = JSON.stringify({ username, password });
    const response = await fetch("/login", { method: "POST", headers, body });
    if (!response.ok) throw new Error(response.statusText);
    // Dont log this as the user password is inside: console.debug(response);
    return { error: null };
  } catch (error) {
    console.log(error);
    return { error }
  }
}

export async function logout(): Promise<any> {
  try {
    const response = await fetch("/logout", { method: "POST", headers });
    if (!response.ok) throw new Error(response.statusText);
    return { error: null };
  } catch (error) {
    return { error: error };
  }
}

export async function checkAuthentication() {
  try {
    const response = await fetch("/session", { method: "POST", headers });
    if (!response.ok)
      if (response.status === 401) return { auth: 0, error: null };
      else throw new Error(response.statusText);
    const data = await response.json();
    console.log(data);
    const last_hash = window.localStorage.getItem("commit_hash");
    if (last_hash) {
      if (last_hash !== data.commit_hash) {
        console.log(`Detected New Backend Version ${data.commit_hash}, Reloading...`);
        window.localStorage.removeItem("commit_hash");
        window.location.reload();
      }
    } else window.localStorage.setItem("commit_hash", data.commit_hash);

    // TODO: Replace with websocket later
    try {
      const res = await apiCall("config.pending.log", {});
      if (res.Error === null) {
        const changelog = res.Data;
      $pendingChangeCount = changelog.length;
      } else console.debug("error fetching pending log", res);
    } catch (ex: any) {
      console.debug("error fetching pending log", ex)
    }

    return { auth: 2, error: null };
  } catch (error) {
    return { auth: 0, error: error };
  }
}
