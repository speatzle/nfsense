<script setup lang="ts">
//import { apiCall } from '../../api';
//import getPlugins from '../../plugins';
//const p = getPlugins();

const logs = $ref([] as any[]); // TODO: Add proper type
let loading = $ref(false);
const websocket: WebSocket | undefined = undefined;

const msgId = $ref(Math.floor(Math.random() * 100000));
const subscriptionId = $ref(0);

const columns = [
  { heading: 'Protocol', path: 'protocol' },
  { heading: 'Source IP', path: 'src_ip' },
  { heading: 'Source Port', path: 'src_port' },
  { heading: 'Destination IP', path: 'dest_ip' },
  { heading: 'Destination Port', path: 'dest_port' },
  { heading: 'Rule', path: 'rule' },
  { heading: 'Timestamp', path: 'timestamp' },
];

async function load(){
  loading = true;
  const websocket = new WebSocket('/api');
  websocket.addEventListener('open', () => {
    console.debug('ws connected', websocket);
    websocket.send(JSON.stringify({ jsonrpc:'2.0', method:'system.logs.fw.live.subscribe', id:msgId }));

    console.debug('ws message sent!');
  });

  websocket.addEventListener('close', (e) => {
    console.debug('DISCONNECTED', e);
  });

  websocket.addEventListener('error', (e) => {
    console.debug('ERROR',e);
  });

  websocket.onmessage = (e) => {
    console.debug('ws message', e);
    // TODO
    const data = JSON.parse(e.data);

    if (data.method === 'system.logs.fw.live.event') {
      const res = data.params.result;
      logs.push({ protocol: res['ip.protocol'], src_ip: res.src_ip, src_port: res.src_port, dest_ip: res.dest_ip, dest_port: res.dest_port, rule: res['oob.prefix'], timestamp: res.timestamp });

    }

    return false;
  };
  loading = false;
}


onMounted(async() => {
  load();
});

onUnmounted(() => {
  // TODO unsubscribe
  // websocket.send(JSON.stringify({ jsonrpc:'2.0', method:'system.log.fw.live.subscribe', id:19899999 }));
  console.debug('closing log websocket');
  websocket?.close();
});

</script>

<template>
  <TableView v-model:data="logs" title="Logs" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
    <button @click="load">Refresh</button>
  </TableView>
</template>
