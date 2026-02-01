<script setup lang="ts">
//import { apiCall } from '../../api';
//import getPlugins from '../../plugins';
//const p = getPlugins();

const logs = $ref([] as any[]); // TODO: Add proper type
let loading = $ref(false);
let websocket: WebSocket | undefined = undefined;

const msgId = $ref(Math.floor(Math.random() * 100000));
let subscriptionId = $ref(0);

const columns = [
  { heading: 'Protocol', path: 'protocol' },
  { heading: 'Source IP', path: 'source_ip' },
  { heading: 'Source Port', path: 'source_port' },
  { heading: 'Destination IP', path: 'destination_ip' },
  { heading: 'Destination Port', path: 'destination_port' },
  { heading: 'Rule', path: 'prefix' },
  { heading: 'Timestamp', path: 'timestamp' },
];

async function load(){
  loading = true;
  websocket = new WebSocket('/api');
  websocket.addEventListener('open', () => {
    console.debug('ws connected', websocket);
    websocket?.send(JSON.stringify({ jsonrpc:'2.0', method:'system.logs.fw.live.subscribe', id:msgId }));

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
    const data = JSON.parse(e.data);

    if (data.method === 'system.logs.fw.live.event') {
      subscriptionId = data.params.subscription;
      logs.push(data.params.result);

    }

    return false;
  };
  loading = false;
}


onMounted(async() => {
  load();
});

onBeforeUnmount(() => {
  websocket?.send(JSON.stringify({ jsonrpc:'2.0', method:'system.log.fw.live.unsubscribe', id:subscriptionId }));
  console.debug('closing log websocket');
  websocket?.close();
});

</script>

<template>
  <TableView v-model:data="logs" title="Logs" :columns="columns" :loading="loading" :table-props="{sort:true, sortSelf: true}">
  </TableView>
</template>
