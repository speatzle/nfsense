<script setup lang="ts">
const props = $defineProps<{entity: string}>();
const { entity } = $(props);

const pageTypes: { [key: string]: any } = {
  "rules": { title: "Rules" },
  "addresses": { title: "Addresses"},
};

let searchTerm = $ref("");
</script>
<template>
  <div>
    <PageHeader :title="pageTypes[entity].title">
      <input class="search-bar" placeholder="Search..." v-model="searchTerm"/>
      <button>
        <i-material-symbols-add/>
      </button>
    </PageHeader>
    <NiceTable :columns="{fname: 'First Name', lname: 'Last Name'}" :sort-self="true" :data="[
      {
        fname: 'Haynes',
        lname: 'Chavez'
      }, {
        fname: 'Brennan',
        lname: 'Bradley'
      }, {
        fname: 'Blanchard',
        lname: 'Thornton'
      }, {
        fname: 'Benjamin',
        lname: 'Nash'
      }, {
        fname: 'Jan',
        lname: 'Bradford'
      }, {
        fname: 'Zelma',
        lname: 'Spears'
      }, {
        fname: 'Freeman',
        lname: 'Page'
      }, {
        fname: 'Wilson',
        lname: 'Carlson'
      }, {
        fname: 'Lewis',
        lname: 'Fuentes'
      }, {
        fname: 'Vega',
        lname: 'Villarreal'
      }, {
        fname: 'Carolyn',
        lname: 'Cardenas'
      }, {
        fname: 'Angie',
        lname: 'Adams'
      }, {
        fname: 'Richards',
        lname: 'Leon'
      }, {
        fname: 'Velma',
        lname: 'Fields'
      }, {
        fname: 'Witt',
        lname: 'Lowe'
      }, {
        fname: 'Waters',
        lname: 'Leblanc'
      }, {
        fname: 'Henry',
        lname: 'Lloyd'
      }, {
        fname: 'Boone',
        lname: 'Greer'
      }, {
        fname: 'Willis',
        lname: 'Stark'
      }, {
        fname: 'Dickson',
        lname: 'Spencer'
      }
    ].filter(x => (`${x.fname} ${x.lname}`).toLowerCase().includes(searchTerm.toLowerCase()))"/>
  </div>
</template>

<style scoped>
.page-content {
  overflow-y: auto;
}
</style>